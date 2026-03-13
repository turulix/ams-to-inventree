# \MetadataApi

All URIs are relative to *https://inv.turulix.de*

Method | HTTP request | Description
------------- | ------------- | -------------
[**metadata_partial_update**](MetadataApi.md#metadata_partial_update) | **PATCH** /api/metadata/{model}/{lookup_field}/{lookup_value}/ | 
[**metadata_pk_partial_update**](MetadataApi.md#metadata_pk_partial_update) | **PATCH** /api/metadata/{model}/{id}/ | 
[**metadata_pk_retrieve**](MetadataApi.md#metadata_pk_retrieve) | **GET** /api/metadata/{model}/{id}/ | 
[**metadata_pk_update**](MetadataApi.md#metadata_pk_update) | **PUT** /api/metadata/{model}/{id}/ | 
[**metadata_retrieve**](MetadataApi.md#metadata_retrieve) | **GET** /api/metadata/{model}/{lookup_field}/{lookup_value}/ | 
[**metadata_update**](MetadataApi.md#metadata_update) | **PUT** /api/metadata/{model}/{lookup_field}/{lookup_value}/ | 



## metadata_partial_update

> metadata_partial_update(lookup_field, lookup_value, model)


Metadata for specific instance; see https://docs.inventree.org/en/stable/plugins/metadata/ for more detail on how metadata works. Most core models support metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookup_field** | **String** |  | [required] |
**lookup_value** | **String** |  | [required] |
**model** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_pk_partial_update

> metadata_pk_partial_update(id, model)


Perform a PATCH request to partially update metadata for the given object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**model** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_pk_retrieve

> metadata_pk_retrieve(id, model)


Perform a GET request to retrieve metadata for the given object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**model** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_pk_update

> metadata_pk_update(id, model)


Perform a PUT request to update metadata for the given object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**model** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_retrieve

> metadata_retrieve(lookup_field, lookup_value, model)


Metadata for specific instance; see https://docs.inventree.org/en/stable/plugins/metadata/ for more detail on how metadata works. Most core models support metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookup_field** | **String** |  | [required] |
**lookup_value** | **String** |  | [required] |
**model** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_update

> metadata_update(lookup_field, lookup_value, model)


Metadata for specific instance; see https://docs.inventree.org/en/stable/plugins/metadata/ for more detail on how metadata works. Most core models support metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookup_field** | **String** |  | [required] |
**lookup_value** | **String** |  | [required] |
**model** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

