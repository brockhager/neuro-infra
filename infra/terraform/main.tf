terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
    google = {
      source  = "hashicorp/google"
      version = "~> 4.0"
    }
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

# AWS
provider "aws" {
  region = var.aws_region
}

resource "aws_ecs_cluster" "neuro_cluster" {
  name = "neuroswarm-cluster"
}

resource "aws_ecs_service" "neuro_gateway" {
  name            = "neuro-gateway"
  cluster         = aws_ecs_cluster.neuro_cluster.id
  task_definition = aws_ecs_task_definition.neuro_gateway.arn
  desired_count   = 3
}

resource "aws_ecs_task_definition" "neuro_gateway" {
  family                   = "neuro-gateway"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = "256"
  memory                   = "512"

  container_definitions = jsonencode([
    {
      name  = "gateway"
      image = "neuroswarm/gateway:latest"
      portMappings = [
        {
          containerPort = 8080
          hostPort      = 8080
        }
      ]
    }
  ])
}

# GCP
provider "google" {
  project = var.gcp_project
  region  = var.gcp_region
}

resource "google_cloud_run_service" "neuro_gateway_gcp" {
  name     = "neuro-gateway"
  location = var.gcp_region

  template {
    spec {
      containers {
        image = "neuroswarm/gateway:latest"
        ports {
          container_port = 8080
        }
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}

# Azure
provider "azurerm" {
  features {}
}

resource "azurerm_resource_group" "neuro" {
  name     = "neuroswarm-rg"
  location = var.azure_location
}

resource "azurerm_container_group" "neuro_gateway" {
  name                = "neuro-gateway"
  location            = azurerm_resource_group.neuro.location
  resource_group_name = azurerm_resource_group.neuro.name
  os_type             = "Linux"

  container {
    name   = "gateway"
    image  = "neuroswarm/gateway:latest"
    cpu    = "0.5"
    memory = "1.0"

    ports {
      port     = 8080
      protocol = "TCP"
    }
  }
}

variable "aws_region" {
  default = "us-east-1"
}

variable "gcp_project" {
  default = "neuroswarm-project"
}

variable "gcp_region" {
  default = "us-central1"
}

variable "azure_location" {
  default = "East US"
}