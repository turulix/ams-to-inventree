# MachineConfigCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **uuid::Uuid** |  | [readonly]
**name** | **String** | Name of machine | 
**machine_type** | **String** | Type of machine | 
**driver** | **String** | Driver used for the machine | 
**initialized** | **bool** | Indicator if machine is initialized. | [readonly]
**active** | Option<**bool**> | Machines can be disabled | [optional]
**status** | **i32** | Numerical machine status if available, else -1. | [readonly]
**status_model** | Option<**String**> | Textual machine status name if available, else None. | [optional][readonly]
**status_text** | **String** | Current status text for machine. | [readonly]
**machine_errors** | **Vec<String>** | List of machine errors. | [readonly]
**is_driver_available** | **bool** | Indicator if driver for machine is available. | [readonly]
**restart_required** | **bool** | Indicator if machine restart is required. | [readonly]
**properties** | [**Vec<models::MachineProperty>**](MachineProperty.md) |  | [readonly][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


