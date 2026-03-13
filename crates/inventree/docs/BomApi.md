# \BomApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bom_bulk_destroy**](BomApi.md#bom_bulk_destroy) | **DELETE** /api/bom/ | 
[**bom_create**](BomApi.md#bom_create) | **POST** /api/bom/ | 
[**bom_destroy**](BomApi.md#bom_destroy) | **DELETE** /api/bom/{id}/ | 
[**bom_list**](BomApi.md#bom_list) | **GET** /api/bom/ | 
[**bom_partial_update**](BomApi.md#bom_partial_update) | **PATCH** /api/bom/{id}/ | 
[**bom_retrieve**](BomApi.md#bom_retrieve) | **GET** /api/bom/{id}/ | 
[**bom_substitute_create**](BomApi.md#bom_substitute_create) | **POST** /api/bom/substitute/ | 
[**bom_substitute_destroy**](BomApi.md#bom_substitute_destroy) | **DELETE** /api/bom/substitute/{id}/ | 
[**bom_substitute_list**](BomApi.md#bom_substitute_list) | **GET** /api/bom/substitute/ | 
[**bom_substitute_partial_update**](BomApi.md#bom_substitute_partial_update) | **PATCH** /api/bom/substitute/{id}/ | 
[**bom_substitute_retrieve**](BomApi.md#bom_substitute_retrieve) | **GET** /api/bom/substitute/{id}/ | 
[**bom_substitute_update**](BomApi.md#bom_substitute_update) | **PUT** /api/bom/substitute/{id}/ | 
[**bom_update**](BomApi.md#bom_update) | **PUT** /api/bom/{id}/ | 
[**bom_validate_partial_update**](BomApi.md#bom_validate_partial_update) | **PATCH** /api/bom/{id}/validate/ | 
[**bom_validate_update**](BomApi.md#bom_validate_update) | **PUT** /api/bom/{id}/validate/ | 



## bom_bulk_destroy

> bom_bulk_destroy(bulk_request)


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


## bom_create

> models::BomItem bom_create(bom_item)


API endpoint for accessing a list of BomItem objects.  - GET: Return list of BomItem objects - POST: Create a new BomItem object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bom_item** | [**BomItem**](BomItem.md) |  | [required] |

### Return type

[**models::BomItem**](BomItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bom_destroy

> bom_destroy(id)


API endpoint for detail view of a single BomItem object.

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


## bom_list

> models::PaginatedBomItemList bom_list(limit, allow_variants, available_stock, can_build, category, consumable, has_pricing, inherited, offset, on_order, optional, ordering, part, part_active, part_detail, part_testable, part_trackable, pricing, search, sub_part_active, sub_part_assembly, sub_part_detail, sub_part_testable, sub_part_trackable, sub_part_virtual, substitutes, uses, validated)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**allow_variants** | Option<**bool**> |  |  |
**available_stock** | Option<**bool**> | Has available stock |  |
**can_build** | Option<**bool**> |  |  |[default to true]
**category** | Option<**i32**> |  |  |
**consumable** | Option<**bool**> |  |  |
**has_pricing** | Option<**bool**> | Has Pricing |  |
**inherited** | Option<**bool**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**on_order** | Option<**bool**> | On order |  |
**optional** | Option<**bool**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**part** | Option<**i32**> |  |  |
**part_active** | Option<**bool**> | Assembly part is active |  |
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to false]
**part_testable** | Option<**bool**> | Assembly part is testable |  |
**part_trackable** | Option<**bool**> | Assembly part is trackable |  |
**pricing** | Option<**bool**> |  |  |[default to false]
**search** | Option<**String**> | A search term. Searched fields: part__IPN, part__description, part__keywords, part__name, part__revision, reference, sub_part__IPN, sub_part__category__name, sub_part__description, sub_part__keywords, sub_part__name, sub_part__revision. |  |
**sub_part_active** | Option<**bool**> | Component part is active |  |
**sub_part_assembly** | Option<**bool**> | Component part is an assembly |  |
**sub_part_detail** | Option<**bool**> |  |  |[default to false]
**sub_part_testable** | Option<**bool**> | Component part is testable |  |
**sub_part_trackable** | Option<**bool**> | Component part is trackable |  |
**sub_part_virtual** | Option<**bool**> | Component part is virtual |  |
**substitutes** | Option<**bool**> |  |  |[default to false]
**uses** | Option<**i32**> |  |  |
**validated** | Option<**bool**> |  |  |

### Return type

[**models::PaginatedBomItemList**](PaginatedBomItemList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bom_partial_update

> models::BomItem bom_partial_update(id, patched_bom_item)


API endpoint for detail view of a single BomItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_bom_item** | Option<[**PatchedBomItem**](PatchedBomItem.md)> |  |  |

### Return type

[**models::BomItem**](BomItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bom_retrieve

> models::BomItem bom_retrieve(id, can_build, part_detail, pricing, sub_part_detail, substitutes)


API endpoint for detail view of a single BomItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**can_build** | Option<**bool**> |  |  |[default to true]
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to false]
**pricing** | Option<**bool**> |  |  |[default to false]
**sub_part_detail** | Option<**bool**> |  |  |[default to false]
**substitutes** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BomItem**](BomItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bom_substitute_create

> models::BomItemSubstitute bom_substitute_create(bom_item_substitute)


API endpoint for accessing a list of BomItemSubstitute objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bom_item_substitute** | [**BomItemSubstitute**](BomItemSubstitute.md) |  | [required] |

### Return type

[**models::BomItemSubstitute**](BomItemSubstitute.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bom_substitute_destroy

> bom_substitute_destroy(id)


API endpoint for detail view of a single BomItemSubstitute object.

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


## bom_substitute_list

> models::PaginatedBomItemSubstituteList bom_substitute_list(limit, bom_item, offset, ordering, part, search)


API endpoint for accessing a list of BomItemSubstitute objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**bom_item** | Option<**i32**> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**part** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedBomItemSubstituteList**](PaginatedBomItemSubstituteList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bom_substitute_partial_update

> models::BomItemSubstitute bom_substitute_partial_update(id, patched_bom_item_substitute)


API endpoint for detail view of a single BomItemSubstitute object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_bom_item_substitute** | Option<[**PatchedBomItemSubstitute**](PatchedBomItemSubstitute.md)> |  |  |

### Return type

[**models::BomItemSubstitute**](BomItemSubstitute.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bom_substitute_retrieve

> models::BomItemSubstitute bom_substitute_retrieve(id)


API endpoint for detail view of a single BomItemSubstitute object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::BomItemSubstitute**](BomItemSubstitute.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bom_substitute_update

> models::BomItemSubstitute bom_substitute_update(id, bom_item_substitute)


API endpoint for detail view of a single BomItemSubstitute object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**bom_item_substitute** | [**BomItemSubstitute**](BomItemSubstitute.md) |  | [required] |

### Return type

[**models::BomItemSubstitute**](BomItemSubstitute.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bom_update

> models::BomItem bom_update(id, bom_item)


API endpoint for detail view of a single BomItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**bom_item** | [**BomItem**](BomItem.md) |  | [required] |

### Return type

[**models::BomItem**](BomItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bom_validate_partial_update

> models::BomItemValidation bom_validate_partial_update(id, patched_bom_item_validation)


API endpoint for validating a BomItem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_bom_item_validation** | Option<[**PatchedBomItemValidation**](PatchedBomItemValidation.md)> |  |  |

### Return type

[**models::BomItemValidation**](BomItemValidation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bom_validate_update

> models::BomItemValidation bom_validate_update(id, bom_item_validation)


API endpoint for validating a BomItem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**bom_item_validation** | Option<[**BomItemValidation**](BomItemValidation.md)> |  |  |

### Return type

[**models::BomItemValidation**](BomItemValidation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

