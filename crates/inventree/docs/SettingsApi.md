# \SettingsApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**settings_global_list**](SettingsApi.md#settings_global_list) | **GET** /api/settings/global/ | 
[**settings_global_partial_update**](SettingsApi.md#settings_global_partial_update) | **PATCH** /api/settings/global/{key}/ | 
[**settings_global_retrieve**](SettingsApi.md#settings_global_retrieve) | **GET** /api/settings/global/{key}/ | 
[**settings_global_update**](SettingsApi.md#settings_global_update) | **PUT** /api/settings/global/{key}/ | 
[**settings_user_list**](SettingsApi.md#settings_user_list) | **GET** /api/settings/user/ | 
[**settings_user_partial_update**](SettingsApi.md#settings_user_partial_update) | **PATCH** /api/settings/user/{key}/ | 
[**settings_user_retrieve**](SettingsApi.md#settings_user_retrieve) | **GET** /api/settings/user/{key}/ | 
[**settings_user_update**](SettingsApi.md#settings_user_update) | **PUT** /api/settings/user/{key}/ | 



## settings_global_list

> models::PaginatedGlobalSettingsList settings_global_list(limit, offset, ordering, search)


API endpoint for accessing a list of global settings objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: key. |  |

### Return type

[**models::PaginatedGlobalSettingsList**](PaginatedGlobalSettingsList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## settings_global_partial_update

> models::GlobalSettings settings_global_partial_update(key, patched_global_settings)


Detail view for an individual \"global setting\" object.  - User must have 'staff' status to view / edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**patched_global_settings** | Option<[**PatchedGlobalSettings**](PatchedGlobalSettings.md)> |  |  |

### Return type

[**models::GlobalSettings**](GlobalSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## settings_global_retrieve

> models::GlobalSettings settings_global_retrieve(key)


Detail view for an individual \"global setting\" object.  - User must have 'staff' status to view / edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**models::GlobalSettings**](GlobalSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## settings_global_update

> models::GlobalSettings settings_global_update(key, global_settings)


Detail view for an individual \"global setting\" object.  - User must have 'staff' status to view / edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**global_settings** | [**GlobalSettings**](GlobalSettings.md) |  | [required] |

### Return type

[**models::GlobalSettings**](GlobalSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## settings_user_list

> models::PaginatedUserSettingsList settings_user_list(limit, offset, ordering, search)


API endpoint for accessing a list of user settings objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: key. |  |

### Return type

[**models::PaginatedUserSettingsList**](PaginatedUserSettingsList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## settings_user_partial_update

> models::UserSettings settings_user_partial_update(key, patched_user_settings)


Detail view for an individual \"user setting\" object.  - User can only view / edit settings their own settings objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**patched_user_settings** | Option<[**PatchedUserSettings**](PatchedUserSettings.md)> |  |  |

### Return type

[**models::UserSettings**](UserSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## settings_user_retrieve

> models::UserSettings settings_user_retrieve(key)


Detail view for an individual \"user setting\" object.  - User can only view / edit settings their own settings objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**models::UserSettings**](UserSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## settings_user_update

> models::UserSettings settings_user_update(key, user_settings)


Detail view for an individual \"user setting\" object.  - User can only view / edit settings their own settings objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**user_settings** | [**UserSettings**](UserSettings.md) |  | [required] |

### Return type

[**models::UserSettings**](UserSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

