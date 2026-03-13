# \MetadataApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**metadata_partial_update**](MetadataApi.md#metadata_partial_update) | **PATCH** /api/metadata/{model}/{lookup_field}/{lookup_value}/ | 
[**metadata_pk_partial_update**](MetadataApi.md#metadata_pk_partial_update) | **PATCH** /api/metadata/{model}/{id}/ | 
[**metadata_pk_retrieve**](MetadataApi.md#metadata_pk_retrieve) | **GET** /api/metadata/{model}/{id}/ | 
[**metadata_pk_update**](MetadataApi.md#metadata_pk_update) | **PUT** /api/metadata/{model}/{id}/ | 
[**metadata_retrieve**](MetadataApi.md#metadata_retrieve) | **GET** /api/metadata/{model}/{lookup_field}/{lookup_value}/ | 
[**metadata_update**](MetadataApi.md#metadata_update) | **PUT** /api/metadata/{model}/{lookup_field}/{lookup_value}/ | 



## metadata_partial_update

> models::Metadata metadata_partial_update(lookup_field, lookup_value, model, patched_metadata)


Metadata for specific instance; see https://docs.inventree.org/en/stable/plugins/metadata/ for more detail on how metadata works. Most core models support metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookup_field** | **String** |  | [required] |
**lookup_value** | **String** |  | [required] |
**model** | **String** |  | [required] |
**patched_metadata** | Option<[**PatchedMetadata**](PatchedMetadata.md)> |  |  |

### Return type

[**models::Metadata**](Metadata.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_pk_partial_update

> models::Metadata metadata_pk_partial_update(id, model, patched_metadata)


Perform a PATCH request to partially update metadata for the given object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**model** | **String** |  | [required] |
**patched_metadata** | Option<[**PatchedMetadata**](PatchedMetadata.md)> |  |  |

### Return type

[**models::Metadata**](Metadata.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_pk_retrieve

> models::Metadata metadata_pk_retrieve(id, model)


Perform a GET request to retrieve metadata for the given object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**model** | **String** |  | [required] |

### Return type

[**models::Metadata**](Metadata.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_pk_update

> models::Metadata metadata_pk_update(id, model, metadata)


Perform a PUT request to update metadata for the given object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**model** | **String** |  | [required] |
**metadata** | [**Metadata**](Metadata.md) |  | [required] |

### Return type

[**models::Metadata**](Metadata.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_retrieve

> models::Metadata metadata_retrieve(lookup_field, lookup_value, model)


Metadata for specific instance; see https://docs.inventree.org/en/stable/plugins/metadata/ for more detail on how metadata works. Most core models support metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookup_field** | **String** |  | [required] |
**lookup_value** | **String** |  | [required] |
**model** | **String** |  | [required] |

### Return type

[**models::Metadata**](Metadata.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_update

> models::Metadata metadata_update(lookup_field, lookup_value, model, metadata)


Metadata for specific instance; see https://docs.inventree.org/en/stable/plugins/metadata/ for more detail on how metadata works. Most core models support metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookup_field** | **String** |  | [required] |
**lookup_value** | **String** |  | [required] |
**model** | **String** |  | [required] |
**metadata** | [**Metadata**](Metadata.md) |  | [required] |

### Return type

[**models::Metadata**](Metadata.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

