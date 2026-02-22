[Sub-Users](/subuser)

Delete sub-user

# Delete a user

This endpoint deletes a user. You can add a query-string parameter `plan_id` in case you want to target a specific plan otherwise it will use the default plan.

## URL Parameters

Parameter| Type| Description  
---|---|---  
`ID`| `int`| The ID of the user to delete  

### Request & Response


    DELETE https://proxy.webshare.io/api/v2/subuser/<ID>/

or in case of targeting a specific plan

    DELETE https://proxy.webshare.io/api/v2/subuser/<ID>/?plan_id=<Plan ID>

PythonJavascriptcURL

example.py

    import requests

    response = requests.delete(
      "https://proxy.webshare.io/api/v2/subuser/<ID>/",
      headers={"Authorization": "Token APIKEY"}
    )

    response.json()

The commands above returns empty response with `204 No Content`

    HTTP/1.1 204 No Content

[Update sub-user](/subuser/update "Update sub-user")[Refresh proxy list of a sub-user](/subuser/refresh_proxy_list "Refresh proxy list of a sub-user")
