# \SelectionApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**selection_create**](SelectionApi.md#selection_create) | **POST** /api/selection/ | 
[**selection_destroy**](SelectionApi.md#selection_destroy) | **DELETE** /api/selection/{id}/ | 
[**selection_entry_create**](SelectionApi.md#selection_entry_create) | **POST** /api/selection/{id}/entry/ | 
[**selection_entry_destroy**](SelectionApi.md#selection_entry_destroy) | **DELETE** /api/selection/{id}/entry/{entrypk}/ | 
[**selection_entry_list**](SelectionApi.md#selection_entry_list) | **GET** /api/selection/{id}/entry/ | 
[**selection_entry_partial_update**](SelectionApi.md#selection_entry_partial_update) | **PATCH** /api/selection/{id}/entry/{entrypk}/ | 
[**selection_entry_retrieve**](SelectionApi.md#selection_entry_retrieve) | **GET** /api/selection/{id}/entry/{entrypk}/ | 
[**selection_entry_update**](SelectionApi.md#selection_entry_update) | **PUT** /api/selection/{id}/entry/{entrypk}/ | 
[**selection_list**](SelectionApi.md#selection_list) | **GET** /api/selection/ | 
[**selection_partial_update**](SelectionApi.md#selection_partial_update) | **PATCH** /api/selection/{id}/ | 
[**selection_retrieve**](SelectionApi.md#selection_retrieve) | **GET** /api/selection/{id}/ | 
[**selection_update**](SelectionApi.md#selection_update) | **PUT** /api/selection/{id}/ | 



## selection_create

> models::SelectionList selection_create(selection_list)


List view for SelectionList objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selection_list** | [**SelectionList**](SelectionList.md) |  | [required] |

### Return type

[**models::SelectionList**](SelectionList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selection_destroy

> selection_destroy(id)


Detail view for a SelectionList object.

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


## selection_entry_create

> models::SelectionEntry selection_entry_create(id, selection_entry)


List view for SelectionEntry objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**selection_entry** | [**SelectionEntry**](SelectionEntry.md) |  | [required] |

### Return type

[**models::SelectionEntry**](SelectionEntry.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selection_entry_destroy

> selection_entry_destroy(entrypk, id)


Detail view for a SelectionEntry object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entrypk** | **i32** |  | [required] |
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selection_entry_list

> models::PaginatedSelectionEntryList selection_entry_list(id, limit, offset)


List view for SelectionEntry objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**models::PaginatedSelectionEntryList**](PaginatedSelectionEntryList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selection_entry_partial_update

> models::SelectionEntry selection_entry_partial_update(entrypk, id, patched_selection_entry)


Detail view for a SelectionEntry object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entrypk** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**patched_selection_entry** | Option<[**PatchedSelectionEntry**](PatchedSelectionEntry.md)> |  |  |

### Return type

[**models::SelectionEntry**](SelectionEntry.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selection_entry_retrieve

> models::SelectionEntry selection_entry_retrieve(entrypk, id)


Detail view for a SelectionEntry object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entrypk** | **i32** |  | [required] |
**id** | **i32** |  | [required] |

### Return type

[**models::SelectionEntry**](SelectionEntry.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selection_entry_update

> models::SelectionEntry selection_entry_update(entrypk, id, selection_entry)


Detail view for a SelectionEntry object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entrypk** | **i32** |  | [required] |
**id** | **i32** |  | [required] |
**selection_entry** | [**SelectionEntry**](SelectionEntry.md) |  | [required] |

### Return type

[**models::SelectionEntry**](SelectionEntry.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selection_list

> models::PaginatedSelectionListList selection_list(limit, offset)


List view for SelectionList objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**models::PaginatedSelectionListList**](PaginatedSelectionListList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selection_partial_update

> models::SelectionList selection_partial_update(id, patched_selection_list)


Detail view for a SelectionList object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_selection_list** | Option<[**PatchedSelectionList**](PatchedSelectionList.md)> |  |  |

### Return type

[**models::SelectionList**](SelectionList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selection_retrieve

> models::SelectionList selection_retrieve(id)


Detail view for a SelectionList object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::SelectionList**](SelectionList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selection_update

> models::SelectionList selection_update(id, selection_list)


Detail view for a SelectionList object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**selection_list** | [**SelectionList**](SelectionList.md) |  | [required] |

### Return type

[**models::SelectionList**](SelectionList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

