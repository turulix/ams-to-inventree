# \SupplierApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**supplier_import_create**](SupplierApi.md#supplier_import_create) | **POST** /api/supplier/import/ | 
[**supplier_list_list**](SupplierApi.md#supplier_list_list) | **GET** /api/supplier/list/ | 
[**supplier_search_list**](SupplierApi.md#supplier_search_list) | **GET** /api/supplier/search/ | 



## supplier_import_create

> models::ImportResult supplier_import_create(import_request)


Import a part by supplier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_request** | [**ImportRequest**](ImportRequest.md) |  | [required] |

### Return type

[**models::ImportResult**](ImportResult.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## supplier_list_list

> Vec<models::SupplierList> supplier_list_list()


List all available supplier plugins.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SupplierList>**](SupplierList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## supplier_search_list

> Vec<models::SearchResult> supplier_search_list(plugin, supplier, term)


Search parts by supplier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** | Plugin slug | [required] |
**supplier** | **String** | Supplier slug | [required] |
**term** | **String** | Search term | [required] |

### Return type

[**Vec<models::SearchResult>**](SearchResult.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

