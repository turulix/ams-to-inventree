# PatchedMachineConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**uuid::Uuid**> |  | [optional][readonly]
**name** | Option<**String**> | Name of machine | [optional]
**machine_type** | Option<**String**> | Type of machine | [optional][readonly]
**driver** | Option<**String**> | Driver used for the machine | [optional][readonly]
**initialized** | Option<**bool**> | Indicator if machine is initialized. | [optional][readonly]
**active** | Option<**bool**> | Machines can be disabled | [optional]
**status** | Option<**i32**> | Numerical machine status if available, else -1. | [optional][readonly]
**status_model** | Option<**String**> | Textual machine status name if available, else None. | [optional][readonly]
**status_text** | Option<**String**> | Current status text for machine. | [optional][readonly]
**machine_errors** | Option<**Vec<String>**> | List of machine errors. | [optional][readonly]
**is_driver_available** | Option<**bool**> | Indicator if driver for machine is available. | [optional][readonly]
**restart_required** | Option<**bool**> | Indicator if machine restart is required. | [optional][readonly]
**properties** | Option<[**Vec<models::MachineProperty>**](MachineProperty.md)> |  | [optional][readonly][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


