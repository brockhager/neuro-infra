#!/usr/bin/env node

// CI check to ensure PDA constants are synchronized between TypeScript and Rust implementations

const fs = require('fs');
const path = require('path');

// Read TypeScript PDA constants
const tsPdaPath = path.join(__dirname, '..', 'neuro-shared', 'src', 'pda.ts');
const tsContent = fs.readFileSync(tsPdaPath, 'utf8');

// Extract TS seeds
const tsSeeds = {};
const tsMatch = tsContent.match(/export const PDA_SEEDS = \{([^}]+)\}/s);
if (tsMatch) {
  const seedLines = tsMatch[1].split('\n').map(line => line.trim()).filter(line => line.includes(':'));
  seedLines.forEach(line => {
    const [key, value] = line.split(':').map(s => s.trim().replace(/['",]/g, ''));
    if (key && value) {
      tsSeeds[key] = value;
    }
  });
}

// Read Rust PDA constants
const rustPdaPath = path.join(__dirname, '..', 'neuro-program', 'src', 'lib.rs');
const rustContent = fs.readFileSync(rustPdaPath, 'utf8');

// Extract Rust seeds
const rustSeeds = {};
const rustMatches = rustContent.match(/pub const [A-Z_]+: &\[u8\] = b"[^"]+"/g);
if (rustMatches) {
  rustMatches.forEach(match => {
    const [constDecl, value] = match.split(' = ');
    const key = constDecl.replace('pub const ', '').replace(': &[u8]', '');
    const seedValue = value.replace('b"', '').replace('"', '');
    rustSeeds[key] = seedValue;
  });
}

// Compare
let hasErrors = false;
console.log('🔍 Checking PDA constant synchronization...\n');

console.log('TypeScript seeds:');
Object.entries(tsSeeds).forEach(([key, value]) => {
  console.log(`  ${key}: "${value}"`);
});

console.log('\nRust seeds:');
Object.entries(rustSeeds).forEach(([key, value]) => {
  console.log(`  ${key}: "${value}"`);
});

console.log('\n🔎 Comparison:');
Object.keys(tsSeeds).forEach(key => {
  if (rustSeeds[key] === tsSeeds[key]) {
    console.log(`✅ ${key}: synchronized`);
  } else {
    console.log(`❌ ${key}: TS="${tsSeeds[key]}" vs Rust="${rustSeeds[key]}"`);
    hasErrors = true;
  }
});

// Check for extra keys
Object.keys(rustSeeds).forEach(key => {
  if (!(key in tsSeeds)) {
    console.log(`⚠️  Extra Rust key: ${key}`);
  }
});

if (hasErrors) {
  console.log('\n❌ PDA constants are out of sync!');
  process.exit(1);
} else {
  console.log('\n✅ All PDA constants are synchronized!');
}