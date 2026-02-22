[Notifications](/notifications)

Restore a notification

# Restore notification

This endpoint lets you restore a notification.

### Parameters

Parameter| Type| Description  
---|---|---  
`ID`| `string`| The ID of the notification to restore.  
  
### Request & Response
    
    
    GET https://proxy.webshare.io/api/v2/notification/<ID>/restore/

PythonJavascriptcURL

restore_notification.py
    
    
    import requests
     
    response = requests.post(
        "https://proxy.webshare.io/api/v2/notification/<ID>/restore/",
        headers={"Authorization": "Token APIKEY"}
    )
    response.json()

The commands above return JSON structured like this:

response.json
    
    
    {
        "id": 13,
        "type": "too_much_bandwidth_too_little_proxies",
        "is_dismissable": true,
        "context": { "plan": 22 },
        "created_at": "2022-06-14T11:58:10.246406-07:00",
        "updated_at": "2022-06-14T11:58:10.246406-07:00",
        "dismissed_at": null,
    }

[Dismiss a notification](/notifications/dismiss "Dismiss a notification")[Proxy List](/proxy-list "Proxy List")
