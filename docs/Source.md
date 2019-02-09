# Source

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type_** | **String** | The type of source to connect to. -  `box` indicates the configuration is to connect an instance of Enterprise Box. -  `salesforce` indicates the configuration is to connect to Salesforce. -  `sharepoint` indicates the configuration is to connect to Microsoft SharePoint Online. -  `web_crawl` indicates the configuration is to perform a web page crawl. | [optional] 
**credential_id** | **String** | The **credential_id** of the credentials to use to connect to the source. Credentials are defined using the **credentials** method. The **source_type** of the credentials used must match the **type** field specified in this object. | [optional] 
**schedule** | [***::models::SourceSchedule**](SourceSchedule.md) |  | [optional] 
**options** | [***::models::SourceOptions**](SourceOptions.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


