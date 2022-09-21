terraform {
  required_version = ">= 1.1.0"
  required_providers {
    kubernetes = {
      source = "hashicorp/kubernetes"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "2.6.0"
    }
    kubectl = {
      source  = "gavinbunney/kubectl"
      version = "1.14.0"
    }
  }
}

provider "helm" {
  kubernetes {
    host                   = var.k8s_config.host
    client_certificate     = base64decode(var.k8s_config.client_certificate)
    client_key             = base64decode(var.k8s_config.client_key)
    cluster_ca_certificate = base64decode(var.k8s_config.cluster_ca_certificate)
  }
}

provider "kubernetes" {
  host                   = var.k8s_config.host
  client_certificate     = base64decode(var.k8s_config.client_certificate)
  client_key             = base64decode(var.k8s_config.client_key)
  cluster_ca_certificate = base64decode(var.k8s_config.cluster_ca_certificate)
}
