resource "kubernetes_namespace" "microstack" {
  metadata {
    name = "microstack"
  }
}
