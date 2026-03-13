# \GenericApi

All URIs are relative to *https://inv.turulix.de*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generic_status_custom_create**](GenericApi.md#generic_status_custom_create) | **POST** /api/generic/status/custom/ | 
[**generic_status_custom_destroy**](GenericApi.md#generic_status_custom_destroy) | **DELETE** /api/generic/status/custom/{id}/ | 
[**generic_status_custom_list**](GenericApi.md#generic_status_custom_list) | **GET** /api/generic/status/custom/ | 
[**generic_status_custom_partial_update**](GenericApi.md#generic_status_custom_partial_update) | **PATCH** /api/generic/status/custom/{id}/ | 
[**generic_status_custom_retrieve**](GenericApi.md#generic_status_custom_retrieve) | **GET** /api/generic/status/custom/{id}/ | 
[**generic_status_custom_update**](GenericApi.md#generic_status_custom_update) | **PUT** /api/generic/status/custom/{id}/ | 
[**generic_status_retrieve**](GenericApi.md#generic_status_retrieve) | **GET** /api/generic/status/{statusmodel}/ | 
[**generic_status_retrieve_all**](GenericApi.md#generic_status_retrieve_all) | **GET** /api/generic/status/ | 



## generic_status_custom_create

> models::CustomState generic_status_custom_create(custom_state)


List view for all custom states.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_state** | [**CustomState**](CustomState.md) |  | [required] |

### Return type

[**models::CustomState**](CustomState.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generic_status_custom_destroy

> generic_status_custom_destroy(id)


Detail view for a particular custom states.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generic_status_custom_list

> models::PaginatedCustomStateList generic_status_custom_list(limit, model, offset, ordering, reference_status, search)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**model** | Option<**i32**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**reference_status** | Option<**String**> |  |  |
**search** | Option<**String**> | A search term. Searched fields: key, label, name, reference_status. |  |

### Return type

[**models::PaginatedCustomStateList**](PaginatedCustomStateList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generic_status_custom_partial_update

> models::CustomState generic_status_custom_partial_update(id, patched_custom_state)


Detail view for a particular custom states.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_custom_state** | Option<[**PatchedCustomState**](PatchedCustomState.md)> |  |  |

### Return type

[**models::CustomState**](CustomState.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generic_status_custom_retrieve

> models::CustomState generic_status_custom_retrieve(id)


Detail view for a particular custom states.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::CustomState**](CustomState.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generic_status_custom_update

> models::CustomState generic_status_custom_update(id, custom_state)


Detail view for a particular custom states.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**custom_state** | [**CustomState**](CustomState.md) |  | [required] |

### Return type

[**models::CustomState**](CustomState.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generic_status_retrieve

> models::GenericStateClass generic_status_retrieve(statusmodel)


Retrieve information about a specific status code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**statusmodel** | **String** |  | [required] |

### Return type

[**models::GenericStateClass**](GenericStateClass.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generic_status_retrieve_all

> std::collections::HashMap<String, serde_json::Value> generic_status_retrieve_all()


Perform a GET request to learn information about status codes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

