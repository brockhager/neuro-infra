terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

provider "aws" {
  region = var.region
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

variable "region" {
  default = "us-east-1"
}