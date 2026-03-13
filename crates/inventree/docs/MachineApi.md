# \MachineApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**machine_create**](MachineApi.md#machine_create) | **POST** /api/machine/ | 
[**machine_destroy**](MachineApi.md#machine_destroy) | **DELETE** /api/machine/{id}/ | 
[**machine_drivers_list**](MachineApi.md#machine_drivers_list) | **GET** /api/machine/drivers/ | 
[**machine_list**](MachineApi.md#machine_list) | **GET** /api/machine/ | 
[**machine_partial_update**](MachineApi.md#machine_partial_update) | **PATCH** /api/machine/{id}/ | 
[**machine_restart_create**](MachineApi.md#machine_restart_create) | **POST** /api/machine/{id}/restart/ | 
[**machine_retrieve**](MachineApi.md#machine_retrieve) | **GET** /api/machine/{id}/ | 
[**machine_settings_list**](MachineApi.md#machine_settings_list) | **GET** /api/machine/{id}/settings/ | 
[**machine_settings_partial_update**](MachineApi.md#machine_settings_partial_update) | **PATCH** /api/machine/{id}/settings/{config_type}/{key}/ | 
[**machine_settings_retrieve**](MachineApi.md#machine_settings_retrieve) | **GET** /api/machine/{id}/settings/{config_type}/{key}/ | 
[**machine_settings_update**](MachineApi.md#machine_settings_update) | **PUT** /api/machine/{id}/settings/{config_type}/{key}/ | 
[**machine_status_retrieve**](MachineApi.md#machine_status_retrieve) | **GET** /api/machine/status/ | 
[**machine_types_list**](MachineApi.md#machine_types_list) | **GET** /api/machine/types/ | 
[**machine_update**](MachineApi.md#machine_update) | **PUT** /api/machine/{id}/ | 



## machine_create

> models::MachineConfigCreate machine_create(machine_config_create)


API endpoint for list of Machine objects.  - GET: Return a list of all Machine objects - POST: create a MachineConfig

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**machine_config_create** | [**MachineConfigCreate**](MachineConfigCreate.md) |  | [required] |

### Return type

[**models::MachineConfigCreate**](MachineConfigCreate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_destroy

> machine_destroy(id)


API detail endpoint for MachineConfig object.  - GET: return a single MachineConfig - PUT: update a MachineConfig - PATCH: partial update a MachineConfig - DELETE: delete a MachineConfig

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_drivers_list

> Vec<models::MachineDriver> machine_drivers_list()


List all machine drivers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MachineDriver>**](MachineDriver.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_list

> models::PaginatedMachineConfigList machine_list(limit, active, driver, machine_type, offset, ordering, search)


API endpoint for list of Machine objects.  - GET: Return a list of all Machine objects - POST: create a MachineConfig

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**active** | Option<**bool**> |  |  |
**driver** | Option<**String**> |  |  |
**machine_type** | Option<**String**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: name. |  |

### Return type

[**models::PaginatedMachineConfigList**](PaginatedMachineConfigList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_partial_update

> models::MachineConfig machine_partial_update(id, patched_machine_config)


API detail endpoint for MachineConfig object.  - GET: return a single MachineConfig - PUT: update a MachineConfig - PATCH: partial update a MachineConfig - DELETE: delete a MachineConfig

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**patched_machine_config** | Option<[**PatchedMachineConfig**](PatchedMachineConfig.md)> |  |  |

### Return type

[**models::MachineConfig**](MachineConfig.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_restart_create

> models::MachineRestart machine_restart_create(id)


Restart machine by pk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::MachineRestart**](MachineRestart.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_retrieve

> models::MachineConfig machine_retrieve(id)


API detail endpoint for MachineConfig object.  - GET: return a single MachineConfig - PUT: update a MachineConfig - PATCH: partial update a MachineConfig - DELETE: delete a MachineConfig

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::MachineConfig**](MachineConfig.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_settings_list

> Vec<models::MachineSetting> machine_settings_list(id)


Return all settings for a machine config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::MachineSetting>**](MachineSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_settings_partial_update

> models::MachineSetting machine_settings_partial_update(config_type, id, key, patched_machine_setting)


Detail endpoint for a machine-specific setting.  - GET: Get machine setting detail - PUT: Update machine setting - PATCH: Update machine setting  (Note that these cannot be created or deleted via API)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_type** | **String** |  | [required] |
**id** | **uuid::Uuid** |  | [required] |
**key** | **String** |  | [required] |
**patched_machine_setting** | Option<[**PatchedMachineSetting**](PatchedMachineSetting.md)> |  |  |

### Return type

[**models::MachineSetting**](MachineSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_settings_retrieve

> models::MachineSetting machine_settings_retrieve(config_type, id, key)


Detail endpoint for a machine-specific setting.  - GET: Get machine setting detail - PUT: Update machine setting - PATCH: Update machine setting  (Note that these cannot be created or deleted via API)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_type** | **String** |  | [required] |
**id** | **uuid::Uuid** |  | [required] |
**key** | **String** |  | [required] |

### Return type

[**models::MachineSetting**](MachineSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_settings_update

> models::MachineSetting machine_settings_update(config_type, id, key, machine_setting)


Detail endpoint for a machine-specific setting.  - GET: Get machine setting detail - PUT: Update machine setting - PATCH: Update machine setting  (Note that these cannot be created or deleted via API)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_type** | **String** |  | [required] |
**id** | **uuid::Uuid** |  | [required] |
**key** | **String** |  | [required] |
**machine_setting** | [**MachineSetting**](MachineSetting.md) |  | [required] |

### Return type

[**models::MachineSetting**](MachineSetting.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_status_retrieve

> models::MachineRegistryStatus machine_status_retrieve()


Provide status data for the machine registry.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MachineRegistryStatus**](MachineRegistryStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_types_list

> Vec<models::MachineType> machine_types_list()


List of all machine types.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MachineType>**](MachineType.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## machine_update

> models::MachineConfig machine_update(id, machine_config)


API detail endpoint for MachineConfig object.  - GET: return a single MachineConfig - PUT: update a MachineConfig - PATCH: partial update a MachineConfig - DELETE: delete a MachineConfig

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**machine_config** | [**MachineConfig**](MachineConfig.md) |  | [required] |

### Return type

[**models::MachineConfig**](MachineConfig.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

