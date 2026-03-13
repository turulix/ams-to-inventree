# \BuildApi

All URIs are relative to *https://inv.turulix.de*

Method | HTTP request | Description
------------- | ------------- | -------------
[**build_allocate_create**](BuildApi.md#build_allocate_create) | **POST** /api/build/{id}/allocate/ | 
[**build_auto_allocate_create**](BuildApi.md#build_auto_allocate_create) | **POST** /api/build/{id}/auto-allocate/ | 
[**build_cancel_create**](BuildApi.md#build_cancel_create) | **POST** /api/build/{id}/cancel/ | 
[**build_complete_create**](BuildApi.md#build_complete_create) | **POST** /api/build/{id}/complete/ | 
[**build_consume_create**](BuildApi.md#build_consume_create) | **POST** /api/build/{id}/consume/ | 
[**build_create**](BuildApi.md#build_create) | **POST** /api/build/ | 
[**build_create_output_create**](BuildApi.md#build_create_output_create) | **POST** /api/build/{id}/create-output/ | 
[**build_delete_outputs_create**](BuildApi.md#build_delete_outputs_create) | **POST** /api/build/{id}/delete-outputs/ | 
[**build_destroy**](BuildApi.md#build_destroy) | **DELETE** /api/build/{id}/ | 
[**build_finish_create**](BuildApi.md#build_finish_create) | **POST** /api/build/{id}/finish/ | 
[**build_hold_create**](BuildApi.md#build_hold_create) | **POST** /api/build/{id}/hold/ | 
[**build_issue_create**](BuildApi.md#build_issue_create) | **POST** /api/build/{id}/issue/ | 
[**build_item_bulk_destroy**](BuildApi.md#build_item_bulk_destroy) | **DELETE** /api/build/item/ | 
[**build_item_create**](BuildApi.md#build_item_create) | **POST** /api/build/item/ | 
[**build_item_destroy**](BuildApi.md#build_item_destroy) | **DELETE** /api/build/item/{id}/ | 
[**build_item_list**](BuildApi.md#build_item_list) | **GET** /api/build/item/ | 
[**build_item_partial_update**](BuildApi.md#build_item_partial_update) | **PATCH** /api/build/item/{id}/ | 
[**build_item_retrieve**](BuildApi.md#build_item_retrieve) | **GET** /api/build/item/{id}/ | 
[**build_item_update**](BuildApi.md#build_item_update) | **PUT** /api/build/item/{id}/ | 
[**build_line_create**](BuildApi.md#build_line_create) | **POST** /api/build/line/ | 
[**build_line_destroy**](BuildApi.md#build_line_destroy) | **DELETE** /api/build/line/{id}/ | 
[**build_line_list**](BuildApi.md#build_line_list) | **GET** /api/build/line/ | 
[**build_line_partial_update**](BuildApi.md#build_line_partial_update) | **PATCH** /api/build/line/{id}/ | 
[**build_line_retrieve**](BuildApi.md#build_line_retrieve) | **GET** /api/build/line/{id}/ | 
[**build_line_update**](BuildApi.md#build_line_update) | **PUT** /api/build/line/{id}/ | 
[**build_list**](BuildApi.md#build_list) | **GET** /api/build/ | 
[**build_partial_update**](BuildApi.md#build_partial_update) | **PATCH** /api/build/{id}/ | 
[**build_retrieve**](BuildApi.md#build_retrieve) | **GET** /api/build/{id}/ | 
[**build_scrap_outputs_create**](BuildApi.md#build_scrap_outputs_create) | **POST** /api/build/{id}/scrap-outputs/ | 
[**build_status_retrieve**](BuildApi.md#build_status_retrieve) | **GET** /api/build/status/ | 
[**build_unallocate_create**](BuildApi.md#build_unallocate_create) | **POST** /api/build/{id}/unallocate/ | 
[**build_update**](BuildApi.md#build_update) | **PUT** /api/build/{id}/ | 



## build_allocate_create

> models::BuildAllocation build_allocate_create(id, build_allocation)


API endpoint to allocate stock items to a build order.  - The BuildOrder object is specified by the URL - Items to allocate are specified as a list called \"items\" with the following options:     - bom_item: pk value of a given BomItem object (must match the part associated with this build)     - stock_item: pk value of a given StockItem object     - quantity: quantity to allocate     - output: StockItem (build order output) to allocate stock against (optional)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_allocation** | [**BuildAllocation**](BuildAllocation.md) |  | [required] |

### Return type

[**models::BuildAllocation**](BuildAllocation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_auto_allocate_create

> models::BuildAutoAllocation build_auto_allocate_create(id, build_auto_allocation)


API endpoint for 'automatically' allocating stock against a build order.  - Only looks at 'untracked' parts - If stock exists in a single location, easy! - If user decides that stock items are \"fungible\", allocate against multiple stock items - If the user wants to, allocate substitute parts if the primary parts are not available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_auto_allocation** | Option<[**BuildAutoAllocation**](BuildAutoAllocation.md)> |  |  |

### Return type

[**models::BuildAutoAllocation**](BuildAutoAllocation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_cancel_create

> models::BuildCancel build_cancel_create(id, build_cancel)


API endpoint for cancelling a BuildOrder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_cancel** | Option<[**BuildCancel**](BuildCancel.md)> |  |  |

### Return type

[**models::BuildCancel**](BuildCancel.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_complete_create

> models::BuildOutputComplete build_complete_create(id, build_output_complete)


API endpoint for completing build outputs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_output_complete** | [**BuildOutputComplete**](BuildOutputComplete.md) |  | [required] |

### Return type

[**models::BuildOutputComplete**](BuildOutputComplete.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_consume_create

> models::BuildConsume build_consume_create(id, build_consume)


API endpoint to consume stock against a build order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_consume** | Option<[**BuildConsume**](BuildConsume.md)> |  |  |

### Return type

[**models::BuildConsume**](BuildConsume.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_create

> models::Build build_create(build)


API endpoint for accessing a list of Build objects.  - GET: Return list of objects (with filters) - POST: Create a new Build object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build** | [**Build**](Build.md) |  | [required] |

### Return type

[**models::Build**](Build.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_create_output_create

> Vec<models::StockItem> build_create_output_create(id, build_output_create)


API endpoint for creating new build output(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_output_create** | [**BuildOutputCreate**](BuildOutputCreate.md) |  | [required] |

### Return type

[**Vec<models::StockItem>**](StockItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_delete_outputs_create

> models::BuildOutputDelete build_delete_outputs_create(id, build_output_delete)


API endpoint for deleting multiple build outputs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_output_delete** | [**BuildOutputDelete**](BuildOutputDelete.md) |  | [required] |

### Return type

[**models::BuildOutputDelete**](BuildOutputDelete.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_destroy

> build_destroy(id)


API endpoint for detail view of a Build object.

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


## build_finish_create

> models::BuildComplete build_finish_create(id, build_complete)


API endpoint for marking a build as finished (completed).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_complete** | Option<[**BuildComplete**](BuildComplete.md)> |  |  |

### Return type

[**models::BuildComplete**](BuildComplete.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_hold_create

> build_hold_create(id)


API endpoint for placing a BuildOrder on hold.

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


## build_issue_create

> build_issue_create(id)


API endpoint for issuing a BuildOrder.

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


## build_item_bulk_destroy

> build_item_bulk_destroy(bulk_request)


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


## build_item_create

> models::BuildItem build_item_create(build_item)


API endpoint for accessing a list of BuildItem objects.  - GET: Return list of objects - POST: Create a new BuildItem object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_item** | [**BuildItem**](BuildItem.md) |  | [required] |

### Return type

[**models::BuildItem**](BuildItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_item_destroy

> build_item_destroy(id)


API endpoint for detail view of a BuildItem object.

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


## build_item_list

> models::PaginatedBuildItemList build_item_list(limit, build, build_detail, build_line, include_variants, install_into, install_into_detail, location, location_detail, offset, ordering, output, part, part_detail, search, stock_detail, stock_item, supplier_part_detail, tracked)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**build** | Option<**i32**> | Build Order |  |
**build_detail** | Option<**bool**> | Include detailed information about the associated build order. |  |[default to false]
**build_line** | Option<**i32**> |  |  |
**include_variants** | Option<**bool**> | Include Variants |  |
**install_into** | Option<**i32**> |  |  |
**install_into_detail** | Option<**bool**> | Include detailed information about the build output for this build item. |  |[default to false]
**location** | Option<**i32**> | Location |  |
**location_detail** | Option<**bool**> | Include detailed information about the location of the allocated stock item. |  |[default to false]
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**output** | Option<**i32**> | Filter by output stock item ID. Use 'null' to find uninstalled build items. |  |
**part** | Option<**i32**> | Part |  |
**part_detail** | Option<**bool**> | Include detailed information about the part associated with this build item. |  |[default to false]
**search** | Option<**String**> | A search term. Searched fields: build_line__bom_item__reference, stock_item__part__IPN, stock_item__part__name, stock_item__supplier_part__SKU. |  |
**stock_detail** | Option<**bool**> | Include detailed information about the allocated stock item. |  |[default to false]
**stock_item** | Option<**i32**> |  |  |
**supplier_part_detail** | Option<**bool**> | Include detailed information about the supplier part associated with this build item. |  |[default to false]
**tracked** | Option<**bool**> | Tracked |  |

### Return type

[**models::PaginatedBuildItemList**](PaginatedBuildItemList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_item_partial_update

> models::BuildItem build_item_partial_update(id, patched_build_item)


API endpoint for detail view of a BuildItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_build_item** | Option<[**PatchedBuildItem**](PatchedBuildItem.md)> |  |  |

### Return type

[**models::BuildItem**](BuildItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_item_retrieve

> models::BuildItem build_item_retrieve(id)


API endpoint for detail view of a BuildItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::BuildItem**](BuildItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_item_update

> models::BuildItem build_item_update(id, build_item)


API endpoint for detail view of a BuildItem object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_item** | [**BuildItem**](BuildItem.md) |  | [required] |

### Return type

[**models::BuildItem**](BuildItem.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_line_create

> models::BuildLine build_line_create(build_line)


API endpoint for accessing a list of BuildLine objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_line** | [**BuildLine**](BuildLine.md) |  | [required] |

### Return type

[**models::BuildLine**](BuildLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_line_destroy

> build_line_destroy(id)


API endpoint for detail view of a BuildLine object.

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


## build_line_list

> models::PaginatedBuildLineList build_line_list(limit, allocated, allocations, assembly, assembly_detail, available, bom_item, bom_item_detail, build, build_detail, consumable, consumed, offset, on_order, optional, order_outstanding, ordering, part, part_detail, search, testable, tracked)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**allocated** | Option<**bool**> | Allocated |  |
**allocations** | Option<**bool**> | Include allocation details showing which stock items are allocated to this build line. |  |[default to false]
**assembly** | Option<**bool**> | Assembly |  |
**assembly_detail** | Option<**bool**> | Include brief details of the assembly (parent part) related to the BOM item in this build line. |  |[default to false]
**available** | Option<**bool**> | Available |  |
**bom_item** | Option<**i32**> |  |  |
**bom_item_detail** | Option<**bool**> | Include detailed information about the BOM item linked to this build line. |  |[default to false]
**build** | Option<**i32**> |  |  |
**build_detail** | Option<**bool**> | Include detailed information about the associated build order. |  |[default to false]
**consumable** | Option<**bool**> | Consumable |  |
**consumed** | Option<**bool**> | Consumed |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**on_order** | Option<**bool**> | On Order |  |
**optional** | Option<**bool**> | Optional |  |
**order_outstanding** | Option<**bool**> | Order Outstanding |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**part** | Option<**i32**> | Part |  |
**part_detail** | Option<**bool**> | Include detailed information about the specific part being built or consumed in this build line. |  |[default to false]
**search** | Option<**String**> | A search term. Searched fields: bom_item__reference, bom_item__sub_part__IPN, bom_item__sub_part__description, bom_item__sub_part__name. |  |
**testable** | Option<**bool**> | Testable |  |
**tracked** | Option<**bool**> | Tracked |  |

### Return type

[**models::PaginatedBuildLineList**](PaginatedBuildLineList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_line_partial_update

> models::BuildLine build_line_partial_update(id, patched_build_line)


API endpoint for detail view of a BuildLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_build_line** | Option<[**PatchedBuildLine**](PatchedBuildLine.md)> |  |  |

### Return type

[**models::BuildLine**](BuildLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_line_retrieve

> models::BuildLine build_line_retrieve(id, allocations, assembly_detail, bom_item_detail, build_detail, part_detail)


API endpoint for detail view of a BuildLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**allocations** | Option<**bool**> | Include allocation details showing which stock items are allocated to this build line. |  |[default to false]
**assembly_detail** | Option<**bool**> | Include brief details of the assembly (parent part) related to the BOM item in this build line. |  |[default to false]
**bom_item_detail** | Option<**bool**> | Include detailed information about the BOM item linked to this build line. |  |[default to false]
**build_detail** | Option<**bool**> | Include detailed information about the associated build order. |  |[default to false]
**part_detail** | Option<**bool**> | Include detailed information about the specific part being built or consumed in this build line. |  |[default to false]

### Return type

[**models::BuildLine**](BuildLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_line_update

> models::BuildLine build_line_update(id, build_line)


API endpoint for detail view of a BuildLine object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_line** | [**BuildLine**](BuildLine.md) |  | [required] |

### Return type

[**models::BuildLine**](BuildLine.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_list

> models::PaginatedBuildList build_list(limit, active, ancestor, assigned_to, assigned_to_me, category, completed_after, completed_before, created_after, created_before, exclude_tree, external, has_project_code, has_start_date, has_target_date, include_variants, issued_by, max_date, min_date, offset, ordering, outstanding, overdue, parent, part, part_detail, project_code, reference, sales_order, search, start_date_after, start_date_before, status, target_date_after, target_date_before)


Override the GET method to determine export options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **i32** | Number of results to return per page. | [required] |
**active** | Option<**bool**> | Build is active |  |
**ancestor** | Option<**i32**> | Ancestor Build |  |
**assigned_to** | Option<**i32**> | Assigned To |  |
**assigned_to_me** | Option<**bool**> | Assigned to me |  |
**category** | Option<**i32**> | Category |  |
**completed_after** | Option<**String**> | Completed after |  |
**completed_before** | Option<**String**> | Completed before |  |
**created_after** | Option<**String**> | Created after |  |
**created_before** | Option<**String**> | Created before |  |
**exclude_tree** | Option<**i32**> | Exclude Tree |  |
**external** | Option<**bool**> |  |  |
**has_project_code** | Option<**bool**> | has_project_code |  |
**has_start_date** | Option<**bool**> | Has start date |  |
**has_target_date** | Option<**bool**> | Has target date |  |
**include_variants** | Option<**bool**> | Include Variants |  |
**issued_by** | Option<**i32**> |  |  |
**max_date** | Option<**String**> | Max Date |  |
**min_date** | Option<**String**> | Min Date |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**outstanding** | Option<**bool**> | Build is outstanding |  |
**overdue** | Option<**bool**> | Build is overdue |  |
**parent** | Option<**i32**> | Parent Build |  |
**part** | Option<**i32**> | Part |  |
**part_detail** | Option<**bool**> | Include detailed information about the related part in the response |  |[default to true]
**project_code** | Option<**i32**> |  |  |
**reference** | Option<**String**> | Filter by exact reference |  |
**sales_order** | Option<**i32**> |  |  |
**search** | Option<**String**> | A search term. Searched fields: part__IPN, part__description, part__name, priority, project_code__code, reference, title. |  |
**start_date_after** | Option<**String**> | Start date after |  |
**start_date_before** | Option<**String**> | Start date before |  |
**status** | Option<**i32**> | Order Status |  |
**target_date_after** | Option<**String**> | Target date after |  |
**target_date_before** | Option<**String**> | Target date before |  |

### Return type

[**models::PaginatedBuildList**](PaginatedBuildList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_partial_update

> models::Build build_partial_update(id, patched_build)


API endpoint for detail view of a Build object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**patched_build** | Option<[**PatchedBuild**](PatchedBuild.md)> |  |  |

### Return type

[**models::Build**](Build.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_retrieve

> models::Build build_retrieve(id)


API endpoint for detail view of a Build object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::Build**](Build.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_scrap_outputs_create

> models::BuildOutputScrap build_scrap_outputs_create(id, build_output_scrap)


API endpoint for scrapping build output(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_output_scrap** | [**BuildOutputScrap**](BuildOutputScrap.md) |  | [required] |

### Return type

[**models::BuildOutputScrap**](BuildOutputScrap.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_status_retrieve

> models::GenericStateClass build_status_retrieve()


Retrieve information about a specific status code

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GenericStateClass**](GenericStateClass.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_unallocate_create

> models::BuildUnallocation build_unallocate_create(id, build_unallocation)


API endpoint for unallocating stock items from a build order.  - The BuildOrder object is specified by the URL - \"output\" (StockItem) can optionally be specified - \"bom_item\" can optionally be specified

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build_unallocation** | Option<[**BuildUnallocation**](BuildUnallocation.md)> |  |  |

### Return type

[**models::BuildUnallocation**](BuildUnallocation.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_update

> models::Build build_update(id, build)


API endpoint for detail view of a Build object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**build** | [**Build**](Build.md) |  | [required] |

### Return type

[**models::Build**](Build.md)

### Authorization

[basicAuth](../README.md#basicAuth), [tokenAuth](../README.md#tokenAuth), [cookieAuth](../README.md#cookieAuth), [oauth2](../README.md#oauth2), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

