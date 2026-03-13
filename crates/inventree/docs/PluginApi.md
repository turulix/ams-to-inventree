# \PluginApi

All URIs are relative to *https://inv.turulix.de*

Method | HTTP request | Description
------------- | ------------- | -------------
[**plugin_kicad_library_plugin_api_category_create**](PluginApi.md#plugin_kicad_library_plugin_api_category_create) | **POST** /plugin/kicad-library-plugin/api/category/ | 
[**plugin_kicad_library_plugin_api_category_destroy**](PluginApi.md#plugin_kicad_library_plugin_api_category_destroy) | **DELETE** /plugin/kicad-library-plugin/api/category/{id}/ | 
[**plugin_kicad_library_plugin_api_category_list**](PluginApi.md#plugin_kicad_library_plugin_api_category_list) | **GET** /plugin/kicad-library-plugin/api/category/ | 
[**plugin_kicad_library_plugin_api_category_partial_update**](PluginApi.md#plugin_kicad_library_plugin_api_category_partial_update) | **PATCH** /plugin/kicad-library-plugin/api/category/{id}/ | 
[**plugin_kicad_library_plugin_api_category_retrieve**](PluginApi.md#plugin_kicad_library_plugin_api_category_retrieve) | **GET** /plugin/kicad-library-plugin/api/category/{id}/ | 
[**plugin_kicad_library_plugin_api_category_update**](PluginApi.md#plugin_kicad_library_plugin_api_category_update) | **PUT** /plugin/kicad-library-plugin/api/category/{id}/ | 
[**plugin_kicad_library_plugin_retrieve**](PluginApi.md#plugin_kicad_library_plugin_retrieve) | **GET** /plugin/kicad-library-plugin/. | 
[**plugin_kicad_library_plugin_v1_categories_list**](PluginApi.md#plugin_kicad_library_plugin_v1_categories_list) | **GET** /plugin/kicad-library-plugin/v1/categories{var}/ | 
[**plugin_kicad_library_plugin_v1_parts_category_json_retrieve**](PluginApi.md#plugin_kicad_library_plugin_v1_parts_category_json_retrieve) | **GET** /plugin/kicad-library-plugin/v1/parts/category/{id}.json | 
[**plugin_kicad_library_plugin_v1_parts_json_retrieve**](PluginApi.md#plugin_kicad_library_plugin_v1_parts_json_retrieve) | **GET** /plugin/kicad-library-plugin/v1/parts/{id}.json | 
[**plugin_kicad_library_plugin_v1_parts_retrieve**](PluginApi.md#plugin_kicad_library_plugin_v1_parts_retrieve) | **GET** /plugin/kicad-library-plugin/v1/parts/. | 
[**plugin_kicad_library_plugin_v1_retrieve**](PluginApi.md#plugin_kicad_library_plugin_v1_retrieve) | **GET** /plugin/kicad-library-plugin/v1/. | 



## plugin_kicad_library_plugin_api_category_create

> models::KicadDetailedCategory plugin_kicad_library_plugin_api_category_create(kicad_detailed_category)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kicad_detailed_category** | Option<[**KicadDetailedCategory**](KicadDetailedCategory.md)> |  |  |

### Return type

[**models::KicadDetailedCategory**](KicadDetailedCategory.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_kicad_library_plugin_api_category_destroy

> plugin_kicad_library_plugin_api_category_destroy(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this KiCad Category. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_kicad_library_plugin_api_category_list

> Vec<models::KicadDetailedCategory> plugin_kicad_library_plugin_api_category_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::KicadDetailedCategory>**](KicadDetailedCategory.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_kicad_library_plugin_api_category_partial_update

> models::KicadDetailedCategory plugin_kicad_library_plugin_api_category_partial_update(id, patched_kicad_detailed_category)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this KiCad Category. | [required] |
**patched_kicad_detailed_category** | Option<[**PatchedKicadDetailedCategory**](PatchedKicadDetailedCategory.md)> |  |  |

### Return type

[**models::KicadDetailedCategory**](KicadDetailedCategory.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_kicad_library_plugin_api_category_retrieve

> models::KicadDetailedCategory plugin_kicad_library_plugin_api_category_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this KiCad Category. | [required] |

### Return type

[**models::KicadDetailedCategory**](KicadDetailedCategory.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_kicad_library_plugin_api_category_update

> models::KicadDetailedCategory plugin_kicad_library_plugin_api_category_update(id, kicad_detailed_category)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this KiCad Category. | [required] |
**kicad_detailed_category** | Option<[**KicadDetailedCategory**](KicadDetailedCategory.md)> |  |  |

### Return type

[**models::KicadDetailedCategory**](KicadDetailedCategory.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_kicad_library_plugin_retrieve

> plugin_kicad_library_plugin_retrieve()


Provide an index of the available endpoints

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_kicad_library_plugin_v1_categories_list

> models::PaginatedKicadCategoryList plugin_kicad_library_plugin_v1_categories_list(limit, var, offset)


List of available KiCad categories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**var** | **String** |  | [required] |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**models::PaginatedKicadCategoryList**](PaginatedKicadCategoryList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_kicad_library_plugin_v1_parts_category_json_retrieve

> plugin_kicad_library_plugin_v1_parts_category_json_retrieve(id)


Preview list for all parts in a given category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_kicad_library_plugin_v1_parts_json_retrieve

> plugin_kicad_library_plugin_v1_parts_json_retrieve(id)


Detailed information endpoint for a single part instance.  Here, the lookup id (pk) is the part id. The custom plugin serializer formats the data into a KiCad compatible format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_kicad_library_plugin_v1_parts_retrieve

> plugin_kicad_library_plugin_v1_parts_retrieve()


Preview list for all parts in a given category

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## plugin_kicad_library_plugin_v1_retrieve

> plugin_kicad_library_plugin_v1_retrieve()


Provide an index of the available endpoints

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

