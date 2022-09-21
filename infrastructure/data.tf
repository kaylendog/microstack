variable "k8s_config" {
  type = object({
    host                   = string
    client_certificate     = string
    client_key             = string
    cluster_ca_certificate = string
  })
}

variable "postgres_config" {
  type = object({
    password = string
  })
}
