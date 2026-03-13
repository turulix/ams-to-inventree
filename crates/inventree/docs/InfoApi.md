# InfoApi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**server** | **String** |  | [readonly]
**id** | Option<**String**> |  | [optional][readonly]
**version** | **String** |  | [readonly]
**instance** | **String** |  | [readonly]
**api_version** | **i32** |  | [readonly]
**worker_running** | **bool** |  | [readonly]
**worker_count** | **i32** |  | [readonly]
**worker_pending_tasks** | **i32** |  | [readonly]
**plugins_enabled** | **bool** |  | [readonly]
**plugins_install_disabled** | **bool** |  | [readonly]
**active_plugins** | Option<**serde_json::Value**> |  | [readonly]
**email_configured** | **bool** |  | [readonly]
**debug_mode** | **bool** |  | [readonly]
**docker_mode** | **bool** |  | [readonly]
**default_locale** | **String** |  | [readonly]
**customize** | [**models::Customize**](Customize.md) |  | [readonly]
**system_health** | **bool** |  | [readonly]
**database** | **String** |  | [readonly]
**platform** | **String** |  | [readonly]
**installer** | **String** |  | [readonly]
**target** | Option<**String**> |  | [optional][readonly]
**django_admin** | **String** |  | [readonly]
**settings** | [**models::Settings**](Settings.md) |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


