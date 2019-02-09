# CredentialDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**credential_type** | **String** | The authentication method for this credentials definition. The  **credential_type** specified must be supported by the **source_type**. The following combinations are possible:  -  `\"source_type\": \"box\"` - valid `credential_type`s: `oauth2` -  `\"source_type\": \"salesforce\"` - valid `credential_type`s: `username_password` -  `\"source_type\": \"sharepoint\"` - valid `credential_type`s: `saml` with **source_version** of `online`, or `ntml_v1` with **source_version** of `2016` -  `\"source_type\": \"web_crawl\"` - valid `credential_type`s: `noauth` or `basic` | [optional] 
**client_id** | **String** | The **client_id** of the source that these credentials connect to. Only valid, and required, with a **credential_type** of `oauth2`. | [optional] 
**enterprise_id** | **String** | The **enterprise_id** of the Box site that these credentials connect to. Only valid, and required, with a **source_type** of `box`. | [optional] 
**url** | **String** | The **url** of the source that these credentials connect to. Only valid, and required, with a **credential_type** of `username_password`, `noauth`, and `basic`. | [optional] 
**username** | **String** | The **username** of the source that these credentials connect to. Only valid, and required, with a **credential_type** of `saml`, `username_password`, `basic`, or `ntml_v1`. | [optional] 
**organization_url** | **String** | The **organization_url** of the source that these credentials connect to. Only valid, and required, with a **credential_type** of `saml`. | [optional] 
**site_collection_path** | **String** | The **site_collection.path** of the source that these credentials connect to. Only valid, and required, with a **source_type** of `sharepoint`. | [optional] 
**client_secret** | **String** | The **client_secret** of the source that these credentials connect to. Only valid, and required, with a **credential_type** of `oauth2`. This value is never returned and is only used when creating or modifying **credentials**. | [optional] 
**public_key_id** | **String** | The **public_key_id** of the source that these credentials connect to. Only valid, and required, with a **credential_type** of `oauth2`. This value is never returned and is only used when creating or modifying **credentials**. | [optional] 
**private_key** | **String** | The **private_key** of the source that these credentials connect to. Only valid, and required, with a **credential_type** of `oauth2`. This value is never returned and is only used when creating or modifying **credentials**. | [optional] 
**passphrase** | **String** | The **passphrase** of the source that these credentials connect to. Only valid, and required, with a **credential_type** of `oauth2`. This value is never returned and is only used when creating or modifying **credentials**. | [optional] 
**password** | **String** | The **password** of the source that these credentials connect to. Only valid, and required, with **credential_type**s of `saml`, `username_password`, `basic`, or `ntml_v1`.   **Note:** When used with a **source_type** of `salesforce`, the password consists of the Salesforce password and a valid Salesforce security token concatenated. This value is never returned and is only used when creating or modifying **credentials**. | [optional] 
**gateway_id** | **String** | The ID of the **gateway** to be connected through (when connecting to intranet sites). Only valid with a **credential_type** of `noauth`, `basic`, or `ntml_v1`. Gateways are created using the `/v1/environments/{environment_id}/gateways` methods. | [optional] 
**source_version** | **String** | The type of Sharepoint repository to connect to. Only valid, and required, with a **source_type** of `sharepoint`. | [optional] 
**web_application_url** | **String** | SharePoint OnPrem WebApplication URL. Only valid, and required, with a **source_version** of `2016`. | [optional] 
**domain** | **String** | The domain used to log in to your OnPrem SharePoint account. Only valid, and required, with a **source_version** of `2016`. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


