# PatchedBuild

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**title** | Option<**String**> | Brief description of the build (optional) | [optional]
**barcode_hash** | Option<**String**> |  | [optional][readonly]
**batch** | Option<**String**> | Batch code for this build output | [optional]
**creation_date** | Option<[**String**](String.md)> |  | [optional][readonly]
**completed** | Option<**i32**> | Number of stock items which have been completed | [optional][readonly]
**completion_date** | Option<[**String**](String.md)> |  | [optional]
**destination** | Option<**i32**> | Select location where the completed items will be stored | [optional]
**external** | Option<**bool**> | This build order is fulfilled externally | [optional]
**parent** | Option<**i32**> | Build Order to which this build is allocated | [optional]
**part** | Option<**i32**> | Select part to build | [optional]
**part_name** | Option<**String**> |  | [optional][readonly]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**project_code** | Option<**i32**> | Project code for this build order | [optional]
**project_code_label** | Option<**String**> |  | [optional][readonly]
**project_code_detail** | Option<[**models::ProjectCode**](ProjectCode.md)> |  | [optional][readonly]
**overdue** | Option<**bool**> |  | [optional][readonly][default to false]
**reference** | Option<**String**> |  | [optional]
**sales_order** | Option<**i32**> | Sales Order to which this build is allocated | [optional]
**quantity** | Option<**f64**> |  | [optional]
**start_date** | Option<[**String**](String.md)> | Scheduled start date for this build order | [optional]
**status** | Option<[**models::BuildStatusEnum**](BuildStatusEnum.md)> | Build status code  * `10` - Pending * `20` - Production * `25` - On Hold * `30` - Cancelled * `40` - Complete | [optional][readonly]
**status_text** | Option<**String**> |  | [optional][readonly]
**status_custom_key** | Option<**i32**> | Additional status information for this item | [optional][readonly]
**target_date** | Option<[**String**](String.md)> | Target date for build completion. Build will be overdue after this date. | [optional]
**take_from** | Option<**i32**> | Select location to take stock from for this build (leave blank to take from any stock location) | [optional]
**notes** | Option<**String**> | Markdown notes (optional) | [optional]
**link** | Option<**String**> | Link to external URL | [optional]
**issued_by** | Option<**i32**> | User who issued this build order | [optional]
**issued_by_detail** | Option<[**models::User**](User.md)> |  | [optional][readonly]
**responsible** | Option<**i32**> | User or group responsible for this build order | [optional]
**responsible_detail** | Option<[**models::Owner**](Owner.md)> |  | [optional][readonly]
**parameters** | Option<[**Vec<models::Parameter>**](Parameter.md)> |  | [optional][readonly]
**priority** | Option<**u32**> | Priority of this build order | [optional]
**level** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


