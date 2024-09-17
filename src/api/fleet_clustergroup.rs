// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -D Default --no-condition -A -d -f -
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
#[kube(
    group = "fleet.cattle.io",
    version = "v1alpha1",
    kind = "ClusterGroup",
    plural = "clustergroups"
)]
#[kube(namespaced)]
#[kube(status = "ClusterGroupStatus")]
#[kube(derive = "Default")]
pub struct ClusterGroupSpec {
    /// Selector is a label selector, used to select clusters for this group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ClusterGroupSelector>,
}

/// Selector is a label selector, used to select clusters for this group.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ClusterGroupSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchExpressions"
    )]
    pub match_expressions: Option<Vec<ClusterGroupSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchLabels"
    )]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ClusterGroupSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ClusterGroupStatus {
    /// ClusterCount is the number of clusters in the cluster group.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "clusterCount"
    )]
    pub cluster_count: Option<i64>,
    /// Conditions is a list of conditions and their statuses for the cluster group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ClusterGroupStatusConditions>>,
    /// Display contains the number of ready, desiredready clusters and a
    /// summary state for the bundle's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ClusterGroupStatusDisplay>,
    /// NonReadyClusterCount is the number of clusters that are not ready.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nonReadyClusterCount"
    )]
    pub non_ready_cluster_count: Option<i64>,
    /// NonReadyClusters is a list of cluster names that are not ready.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nonReadyClusters"
    )]
    pub non_ready_clusters: Option<Vec<String>>,
    /// ResourceCounts contains the number of resources in each state over
    /// all bundles in the cluster group.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "resourceCounts"
    )]
    pub resource_counts: Option<ClusterGroupStatusResourceCounts>,
    /// Summary is a summary of the bundle deployments and their resources
    /// in the cluster group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<ClusterGroupStatusSummary>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ClusterGroupStatusConditions {
    /// Last time the condition transitioned from one status to another.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastTransitionTime"
    )]
    pub last_transition_time: Option<String>,
    /// The last time this condition was updated.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastUpdateTime"
    )]
    pub last_update_time: Option<String>,
    /// Human-readable message indicating details about last transition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type of cluster condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Display contains the number of ready, desiredready clusters and a
/// summary state for the bundle's resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ClusterGroupStatusDisplay {
    /// ReadyBundles is a string in the form "%d/%d", that describes the
    /// number of bundles that are ready vs. the number of bundles desired
    /// to be ready.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "readyBundles"
    )]
    pub ready_bundles: Option<String>,
    /// ReadyClusters is a string in the form "%d/%d", that describes the
    /// number of clusters that are ready vs. the number of clusters desired
    /// to be ready.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "readyClusters"
    )]
    pub ready_clusters: Option<String>,
    /// State is a summary state for the cluster group, showing "NotReady" if
    /// there are non-ready resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// ResourceCounts contains the number of resources in each state over
/// all bundles in the cluster group.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ClusterGroupStatusResourceCounts {
    /// DesiredReady is the number of resources that should be ready.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "desiredReady"
    )]
    pub desired_ready: Option<i64>,
    /// Missing is the number of missing resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<i64>,
    /// Modified is the number of resources that have been modified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<i64>,
    /// NotReady is the number of not ready resources. Resources are not
    /// ready if they do not match any other state.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notReady")]
    pub not_ready: Option<i64>,
    /// Orphaned is the number of orphaned resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orphaned: Option<i64>,
    /// Ready is the number of ready resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<i64>,
    /// Unknown is the number of resources in an unknown state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i64>,
    /// WaitApplied is the number of resources that are waiting to be applied.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "waitApplied"
    )]
    pub wait_applied: Option<i64>,
}

/// Summary is a summary of the bundle deployments and their resources
/// in the cluster group.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ClusterGroupStatusSummary {
    /// DesiredReady is the number of bundle deployments that should be
    /// ready.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "desiredReady"
    )]
    pub desired_ready: Option<i64>,
    /// ErrApplied is the number of bundle deployments that have been synced
    /// from the Fleet controller and the downstream cluster, but with some
    /// errors when deploying the bundle.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "errApplied"
    )]
    pub err_applied: Option<i64>,
    /// Modified is the number of bundle deployments that have been deployed
    /// and for which all resources are ready, but where some changes from the
    /// Git repository have not yet been synced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<i64>,
    /// NonReadyClusters is a list of states, which is filled for a bundle
    /// that is not ready.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nonReadyResources"
    )]
    pub non_ready_resources: Option<Vec<ClusterGroupStatusSummaryNonReadyResources>>,
    /// NotReady is the number of bundle deployments that have been deployed
    /// where some resources are not ready.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notReady")]
    pub not_ready: Option<i64>,
    /// OutOfSync is the number of bundle deployments that have been synced
    /// from Fleet controller, but not yet by the downstream agent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outOfSync")]
    pub out_of_sync: Option<i64>,
    /// Pending is the number of bundle deployments that are being processed
    /// by Fleet controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    /// Ready is the number of bundle deployments that have been deployed
    /// where all resources are ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<i64>,
    /// WaitApplied is the number of bundle deployments that have been
    /// synced from Fleet controller and downstream cluster, but are waiting
    /// to be deployed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "waitApplied"
    )]
    pub wait_applied: Option<i64>,
}

/// NonReadyResource contains information about a bundle that is not ready for a
/// given state like "ErrApplied". It contains a list of non-ready or modified
/// resources and their states.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ClusterGroupStatusSummaryNonReadyResources {
    /// State is the state of the resource, like e.g. "NotReady" or "ErrApplied".
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bundleState"
    )]
    pub bundle_state: Option<String>,
    /// Message contains information why the bundle is not ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// ModifiedStatus lists the state for each modified resource.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "modifiedStatus"
    )]
    pub modified_status: Option<Vec<ClusterGroupStatusSummaryNonReadyResourcesModifiedStatus>>,
    /// Name is the name of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// NonReadyStatus lists the state for each non-ready resource.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "nonReadyStatus"
    )]
    pub non_ready_status: Option<Vec<ClusterGroupStatusSummaryNonReadyResourcesNonReadyStatus>>,
}

/// ModifiedStatus is used to report the status of a resource that is modified.
/// It indicates if the modification was a create, a delete or a patch.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ClusterGroupStatusSummaryNonReadyResourcesModifiedStatus {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "apiVersion"
    )]
    pub api_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<String>,
}

/// NonReadyStatus is used to report the status of a resource that is not ready. It includes a summary.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ClusterGroupStatusSummaryNonReadyResourcesNonReadyStatus {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "apiVersion"
    )]
    pub api_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<ClusterGroupStatusSummaryNonReadyResourcesNonReadyStatusSummary>,
    /// UID is a type that holds unique ID values, including UUIDs.  Because we
    /// don't ONLY use UUIDs, this is an alias to string.  Being a type captures
    /// intent and helps make sure that UIDs and names do not get conflated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct ClusterGroupStatusSummaryNonReadyResourcesNonReadyStatusSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitioning: Option<bool>,
}
