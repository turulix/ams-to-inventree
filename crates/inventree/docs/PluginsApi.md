# \PluginsApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**plugins_activate_partial_update**](PluginsApi.md#plugins_activate_partial_update) | **PATCH** /api/plugins/{plugin}/activate/ | 
[**plugins_activate_update**](PluginsApi.md#plugins_activate_update) | **PUT** /api/plugins/{plugin}/activate/ | 
[**plugins_admin_retrieve**](PluginsApi.md#plugins_admin_retrieve) | **GET** /api/plugins/{plugin}/admin/ | 
[**plugins_destroy**](PluginsApi.md#plugins_destroy) | **DELETE** /api/plugins/{plugin}/ | 
[**plugins_install_create**](PluginsApi.md#plugins_install_create) | **POST** /api/plugins/install/ | 
[**plugins_list**](PluginsApi.md#plugins_list) | **GET** /api/plugins/ | 
[**plugins_reload_create**](PluginsApi.md#plugins_reload_create) | **POST** /api/plugins/reload/ | 
[**plugins_retrieve**](PluginsApi.md#plugins_retrieve) | **GET** /api/plugins/{plugin}/ | 
[**plugins_settings_list**](PluginsApi.md#plugins_settings_list) | **GET** /api/plugins/{plugin}/settings/ | 
[**plugins_settings_list_all**](PluginsApi.md#plugins_settings_list_all) | **GET** /api/plugins/settings/ | 
[**plugins_settings_partial_update**](PluginsApi.md#plugins_settings_partial_update) | **PATCH** /api/plugins/{plugin}/settings/{key}/ | 
[**plugins_settings_retrieve**](PluginsApi.md#plugins_settings_retrieve) | **GET** /api/plugins/{plugin}/settings/{key}/ | 
[**plugins_settings_update**](PluginsApi.md#plugins_settings_update) | **PUT** /api/plugins/{plugin}/settings/{key}/ | 
[**plugins_status_retrieve**](PluginsApi.md#plugins_status_retrieve) | **GET** /api/plugins/status/ | 
[**plugins_ui_features_list**](PluginsApi.md#plugins_ui_features_list) | **GET** /api/plugins/ui/features/{feature}/ | 
[**plugins_uninstall_partial_update**](PluginsApi.md#plugins_uninstall_partial_update) | **PATCH** /api/plugins/{plugin}/uninstall/ | 
[**plugins_uninstall_update**](PluginsApi.md#plugins_uninstall_update) | **PUT** /api/plugins/{plugin}/uninstall/ | 
[**plugins_user_settings_list**](PluginsApi.md#plugins_user_settings_list) | **GET** /api/plugins/{plugin}/user-settings/ | 
[**plugins_user_settings_partial_update**](PluginsApi.md#plugins_user_settings_partial_update) | **PATCH** /api/plugins/{plugin}/user-settings/{key}/ | 
[**plugins_user_settings_retrieve**](PluginsApi.md#plugins_user_settings_retrieve) | **GET** /api/plugins/{plugin}/user-settings/{key}/ | 
[**plugins_user_settings_update**](PluginsApi.md#plugins_user_settings_update) | **PUT** /api/plugins/{plugin}/user-settings/{key}/ | 



## plugins_activate_partial_update

> models::PluginActivate plugins_activate_partial_update(plugin, patched_plugin_activate)


Endpoint for activating a plugin.  - PATCH: Activate a plugin  Pass a boolean value for the 'active' field. If not provided, it is assumed to be True, and the plugin will be activated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** |  | [required] |
**patched_plugin_activate** | Option<[**PatchedPluginActivate**](PatchedPluginActivate.md)> |  |  |

### Return type

[**models::PluginActivate**](PluginActivate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_activate_update

> models::PluginActivate plugins_activate_update(plugin, plugin_activate)


Endpoint for activating a plugin.  - PATCH: Activate a plugin  Pass a boolean value for the 'active' field. If not provided, it is assumed to be True, and the plugin will be activated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** |  | [required] |
**plugin_activate** | Option<[**PluginActivate**](PluginActivate.md)> |  |  |

### Return type

[**models::PluginActivate**](PluginActivate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_admin_retrieve

> models::PluginAdminDetail plugins_admin_retrieve(plugin)


Endpoint for viewing admin integration plugin details.  This endpoint is used to view the available admin integration options for a plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** |  | [required] |

### Return type

[**models::PluginAdminDetail**](PluginAdminDetail.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_destroy

> plugins_destroy(plugin)


Handle DELETE request for a PluginConfig instance.  We only allow plugin deletion if the plugin is not active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_install_create

> models::PluginConfigInstall plugins_install_create(plugin_config_install)


Endpoint for installing a new plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_config_install** | [**PluginConfigInstall**](PluginConfigInstall.md) |  | [required] |

### Return type

[**models::PluginConfigInstall**](PluginConfigInstall.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_list

> models::PaginatedPluginConfigList plugins_list(limit, active, builtin, installed, mandatory, mixin, offset, ordering, sample, search)


API endpoint for list of PluginConfig objects.  - GET: Return a list of all PluginConfig objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**active** | Option<**bool**> |  |  |
**builtin** | Option<**bool**> | Builtin |  |
**installed** | Option<**bool**> | Installed |  |
**mandatory** | Option<**bool**> | Mandatory |  |
**mixin** | Option<**String**> | Mixin |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**sample** | Option<**bool**> | Sample |  |
**search** | Option<**String**> | A search term. Searched fields: key, name. |  |

### Return type

[**models::PaginatedPluginConfigList**](PaginatedPluginConfigList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_reload_create

> models::PluginReload plugins_reload_create(plugin_reload)


Endpoint for reloading all plugins.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_reload** | Option<[**PluginReload**](PluginReload.md)> |  |  |

### Return type

[**models::PluginReload**](PluginReload.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_retrieve

> models::PluginConfig plugins_retrieve(plugin)


API detail endpoint for PluginConfig object.  get: Return a single PluginConfig object  post: Update a PluginConfig  delete: Remove a PluginConfig

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** |  | [required] |

### Return type

[**models::PluginConfig**](PluginConfig.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_settings_list

> Vec<models::PluginSetting> plugins_settings_list(plugin)


Get all settings for a plugin config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** |  | [required] |

### Return type

[**Vec<models::PluginSetting>**](PluginSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_settings_list_all

> models::PaginatedPluginSettingList plugins_settings_list_all(limit, offset, plugin__active, plugin__key)


List endpoint for all plugin related settings.  - read only - only accessible by staff users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**plugin__active** | Option<**bool**> |  |  |
**plugin__key** | Option<**String**> |  |  |

### Return type

[**models::PaginatedPluginSettingList**](PaginatedPluginSettingList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_settings_partial_update

> models::PluginSetting plugins_settings_partial_update(key, plugin, patched_plugin_setting)


Detail endpoint for a plugin-specific setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**plugin** | **String** |  | [required] |
**patched_plugin_setting** | Option<[**PatchedPluginSetting**](PatchedPluginSetting.md)> |  |  |

### Return type

[**models::PluginSetting**](PluginSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_settings_retrieve

> models::PluginSetting plugins_settings_retrieve(key, plugin)


Detail endpoint for a plugin-specific setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**plugin** | **String** |  | [required] |

### Return type

[**models::PluginSetting**](PluginSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_settings_update

> models::PluginSetting plugins_settings_update(key, plugin, plugin_setting)


Detail endpoint for a plugin-specific setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**plugin** | **String** |  | [required] |
**plugin_setting** | [**PluginSetting**](PluginSetting.md) |  | [required] |

### Return type

[**models::PluginSetting**](PluginSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_status_retrieve

> models::PluginRegistryStatus plugins_status_retrieve()


Show plugin registry status information.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PluginRegistryStatus**](PluginRegistryStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_ui_features_list

> Vec<models::PluginUiFeature> plugins_ui_features_list(feature)


Show available plugin ui features.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature** | **String** |  | [required] |

### Return type

[**Vec<models::PluginUiFeature>**](PluginUIFeature.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_uninstall_partial_update

> models::PluginUninstall plugins_uninstall_partial_update(plugin, patched_plugin_uninstall)


Endpoint for uninstalling a single plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** |  | [required] |
**patched_plugin_uninstall** | Option<[**PatchedPluginUninstall**](PatchedPluginUninstall.md)> |  |  |

### Return type

[**models::PluginUninstall**](PluginUninstall.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_uninstall_update

> models::PluginUninstall plugins_uninstall_update(plugin, plugin_uninstall)


Endpoint for uninstalling a single plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** |  | [required] |
**plugin_uninstall** | Option<[**PluginUninstall**](PluginUninstall.md)> |  |  |

### Return type

[**models::PluginUninstall**](PluginUninstall.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_user_settings_list

> Vec<models::PluginUserSetting> plugins_user_settings_list(plugin)


Get all user settings for a plugin config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** |  | [required] |

### Return type

[**Vec<models::PluginUserSetting>**](PluginUserSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_user_settings_partial_update

> models::PluginUserSetting plugins_user_settings_partial_update(key, plugin, patched_plugin_user_setting)


Detail endpoint for a plugin-specific user setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**plugin** | **String** |  | [required] |
**patched_plugin_user_setting** | Option<[**PatchedPluginUserSetting**](PatchedPluginUserSetting.md)> |  |  |

### Return type

[**models::PluginUserSetting**](PluginUserSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_user_settings_retrieve

> models::PluginUserSetting plugins_user_settings_retrieve(key, plugin)


Detail endpoint for a plugin-specific user setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**plugin** | **String** |  | [required] |

### Return type

[**models::PluginUserSetting**](PluginUserSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugins_user_settings_update

> models::PluginUserSetting plugins_user_settings_update(key, plugin, plugin_user_setting)


Detail endpoint for a plugin-specific user setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**plugin** | **String** |  | [required] |
**plugin_user_setting** | [**PluginUserSetting**](PluginUserSetting.md) |  | [required] |

### Return type

[**models::PluginUserSetting**](PluginUserSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

