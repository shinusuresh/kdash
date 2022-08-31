use k8s_openapi::{
  api::rbac::v1::{ClusterRole, ClusterRoleBinding, Role, RoleBinding},
  chrono::Utc,
};

use super::{models::KubeResource, utils};

#[derive(Clone, Debug, PartialEq)]
pub struct KubeRoles {
  pub namespace: String,
  pub name: String,
  pub age: String,
  k8s_obj: Role,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KubeRoleBindings {
  pub namespace: String,
  pub name: String,
  pub role: String,
  pub age: String,
  k8s_obj: RoleBinding,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KubeClusterRoles {
  pub name: String,
  pub age: String,
  k8s_obj: ClusterRole,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KubeClusterRoleBinding {
  pub name: String,
  pub role: String,
  pub age: String,
  k8s_obj: ClusterRoleBinding,
}

impl KubeResource<Role> for KubeRoles {
  fn from_api(role: &Role) -> Self {
    KubeRoles {
      namespace: role.metadata.namespace.clone().unwrap_or_default(),
      name: role.metadata.name.clone().unwrap_or_default(),
      age: utils::to_age(role.metadata.creation_timestamp.as_ref(), Utc::now()),
      k8s_obj: role.to_owned(),
    }
  }

  fn get_k8s_obj(&self) -> &Role {
    &self.k8s_obj
  }
}

impl KubeResource<ClusterRole> for KubeClusterRoles {
  fn from_api(clusterrole: &ClusterRole) -> Self {
    KubeClusterRoles {
      name: clusterrole.metadata.name.clone().unwrap_or_default(),
      age: utils::to_age(clusterrole.metadata.creation_timestamp.as_ref(), Utc::now()),
      k8s_obj: clusterrole.to_owned(),
    }
  }

  fn get_k8s_obj(&self) -> &ClusterRole {
    &self.k8s_obj
  }
}

impl KubeResource<RoleBinding> for KubeRoleBindings {
  fn from_api(rolebinding: &RoleBinding) -> Self {
    KubeRoleBindings {
      namespace: rolebinding.metadata.namespace.clone().unwrap_or_default(),
      name: rolebinding.metadata.name.clone().unwrap_or_default(),
      role: rolebinding.role_ref.name.clone(),
      age: utils::to_age(rolebinding.metadata.creation_timestamp.as_ref(), Utc::now()),
      k8s_obj: rolebinding.to_owned(),
    }
  }

  fn get_k8s_obj(&self) -> &RoleBinding {
    &self.k8s_obj
  }
}

impl KubeResource<ClusterRoleBinding> for KubeClusterRoleBinding {
  fn from_api(clusterrolebinding: &ClusterRoleBinding) -> Self {
    KubeClusterRoleBinding {
      name: clusterrolebinding.metadata.name.clone().unwrap_or_default(),
      role: format!(
        "{}/{}",
        clusterrolebinding.role_ref.kind.clone(),
        clusterrolebinding.role_ref.name.clone()
      ),
      age: utils::to_age(
        clusterrolebinding.metadata.creation_timestamp.as_ref(),
        Utc::now(),
      ),
      k8s_obj: clusterrolebinding.to_owned(),
    }
  }

  fn get_k8s_obj(&self) -> &ClusterRoleBinding {
    &self.k8s_obj
  }
}

#[cfg(test)]
mod tests {
  use k8s_openapi::chrono::Utc;

  use crate::app::{
    roles::{KubeClusterRoles, KubeClusterRoleBinding, KubeRoleBindings, KubeRoles},
    test_utils::{convert_resource_from_file, get_time},
    utils,
  };

  #[test]
  fn test_roles_binding_from_rbac_api() {
    let (roles, roles_list): (Vec<KubeRoles>, Vec<_>) = convert_resource_from_file("roles");

    assert_eq!(roles.len(), 1);
    assert_eq!(
      roles[0],
      KubeRoles {
        namespace: "default".to_string(),
        name: "kiali-viewer".into(),
        age: utils::to_age(Some(&get_time("2022-06-27T16:33:06Z")), Utc::now()),
        k8s_obj: roles_list[0].clone(),
      }
    )
  }

  #[test]
  fn test_cluster_roles_from_rbac_api() {
    let (clusterroles, cluster_roles_list): (Vec<KubeClusterRoles>, Vec<_>) =
      convert_resource_from_file("clusterroles");

    assert_eq!(clusterroles.len(), 1);
    assert_eq!(
      clusterroles[0],
      KubeClusterRoles {
        name: "admin".into(),
        age: utils::to_age(Some(&get_time("2021-12-14T11:04:22Z")), Utc::now()),
        k8s_obj: cluster_roles_list[0].clone(),
      }
    )
  }

  #[test]
  fn test_role_binding_from_rbac_api() {
    let (rolebindings, rolebindings_list): (Vec<KubeRoleBindings>, Vec<_>) =
      convert_resource_from_file("role_bindings");

    assert_eq!(rolebindings.len(), 1);
    assert_eq!(
      rolebindings[0],
      KubeRoleBindings {
        namespace: "default".to_string(),
        name: "kiali".into(),
        role: "kiali-viewer".into(),
        age: utils::to_age(Some(&get_time("2022-06-27T16:33:07Z")), Utc::now()),
        k8s_obj: rolebindings_list[0].clone(),
      }
    )
  }
  
  fn test_cluster_role_bindings_from_rbac_api() {
    let (clusterrolebinding, cluster_role_bindings_list): (Vec<KubeClusterRoleBinding>, Vec<_>) =
      convert_resource_from_file("clusterrole_binding");

    assert_eq!(clusterrolebinding.len(), 2);
    assert_eq!(
      clusterrolebinding[0],
      KubeClusterRoleBinding {
        name: "admin-user".into(),
        role: "ClusterRole/cluster-admin".into(),
        age: utils::to_age(Some(&get_time("2022-03-02T16:50:53Z")), Utc::now()),
        k8s_obj: cluster_role_bindings_list[0].clone(),
      }
    )
  }
}