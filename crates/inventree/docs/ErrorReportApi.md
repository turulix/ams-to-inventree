# \ErrorReportApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**error_report_bulk_destroy**](ErrorReportApi.md#error_report_bulk_destroy) | **DELETE** /api/error-report/ | 
[**error_report_destroy**](ErrorReportApi.md#error_report_destroy) | **DELETE** /api/error-report/{id}/ | 
[**error_report_list**](ErrorReportApi.md#error_report_list) | **GET** /api/error-report/ | 
[**error_report_partial_update**](ErrorReportApi.md#error_report_partial_update) | **PATCH** /api/error-report/{id}/ | 
[**error_report_retrieve**](ErrorReportApi.md#error_report_retrieve) | **GET** /api/error-report/{id}/ | 
[**error_report_update**](ErrorReportApi.md#error_report_update) | **PUT** /api/error-report/{id}/ | 



## error_report_bulk_destroy

> error_report_bulk_destroy(bulk_request)


Perform a DELETE operation against this list endpoint.  Note that the typical DRF list endpoint does not support DELETE, so this method is provided as a custom implementation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_request** | [**BulkRequest**](BulkRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## error_report_destroy

> error_report_destroy(id)


Detail view for a single error message.

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


## error_report_list

> models::PaginatedErrorMessageList error_report_list(limit, offset, ordering, search)


List view for server error messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**search** | Option<**String**> | A search term. Searched fields: data, info. |  |

### Return type

[**models::PaginatedErrorMessageList**](PaginatedErrorMessageList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## error_report_partial_update

> models::ErrorMessage error_report_partial_update(id, patched_error_message)


Detail view for a single error message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_error_message** | Option<[**PatchedErrorMessage**](PatchedErrorMessage.md)> |  |  |

### Return type

[**models::ErrorMessage**](ErrorMessage.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## error_report_retrieve

> models::ErrorMessage error_report_retrieve(id)


Detail view for a single error message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ErrorMessage**](ErrorMessage.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## error_report_update

> models::ErrorMessage error_report_update(id, error_message)


Detail view for a single error message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**error_message** | Option<[**ErrorMessage**](ErrorMessage.md)> |  |  |

### Return type

[**models::ErrorMessage**](ErrorMessage.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

