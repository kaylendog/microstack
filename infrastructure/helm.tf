resource "helm_release" "consul" {
  name       = "consul"
  repository = "https://helm.releases.hashicorp.com"
  chart      = "consul"
  namespace  = "microstack"
  values = [
    file("${path.module}/helm/consul.yml")
  ]
}

// load consul intentions
resource "kubectl_manifest" "consul-intentions" {
  yaml_body = file("${path.module}/kubernetes/intentions.yml")
  depends_on = [
	helm_release.consul
  ]
}

resource "helm_release" "postgres" {
  name       = "postgres"
  repository = "https://charts.bitnami.com/bitnami"
  chart      = "postgresql"
  namespace  = "microstack"

  set {
    name  = "global.postgresql.auth.postgresPassword"
    value = var.postgres_config.password
  }
}
