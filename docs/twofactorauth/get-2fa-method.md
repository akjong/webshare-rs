[Two-Factor Auth](/twofactorauth)

Get 2FA Method

## Get 2FA Method

Every account by default has 2FA with Email enabled. This API lets you get the active 2FA method for your account.
    
    
    GET https://proxy.webshare.io/api/v2/twofactorauth/method/current/

### Request & Response

PythonJavascriptcURL

change_email_request.py
    
    
    import requests
     
    response = requests.post(
        "https://proxy.webshare.io/api/v2/twofactorauth/method/current/",
        json={
            "password": "newpassword1234",
            "new_email": "newemail@webshare.io"
        },
        headers={"Authorization": "Token APIKEY"}
    )
     
    assert response.status_code == 204

the commands above return JSON structured like this:

response.json
    
    
    {
      "id": 137,
      "type": "device_totp",
      "active": True,
      "created_at": "2023-03-04T05:34:35.553059-08:00",
      "updated_at": "2023-03-04T05:34:35.553059-08:00",
    }

[Enter 2FA Code](/twofactorauth/enter-2fa-code "Enter 2FA Code")[Change 2FA Method](/twofactorauth/change-2fa-method "Change 2FA Method")
