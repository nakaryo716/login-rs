# 認証システム
Axum Webフレームワークでのログインの実装の確認(mock model)  
axum-extraクレートを用いてCookieのやり取りを行っている。  
かなり適当なtodoアプリ  

## Endpoints
- get method : "sign/:id        => サインイン
- post method: "/user/create    => サインアップ
- post method: "/todo           => todo作成 (ユーザ認証が必要)
- get method : "/todo"          => 全ユーザーの全todoを取得

## Getting Start
1. http://localhost:3000/user/create  
    にアクセスし、新規ユーザのためのJson bodyを送る
    ```json
    {
        "user_name": "ferris"
    }
    ```
    この時のレスポンスは以下のようにidが付与される  
    ```json
    {
        "user_id": "5",
        "user_name": "ferris"
    }
    ```
2. http://localhost:3000/sign/5  
    にアクセスするとCookieが発行される。
    ```
    test_session_id = $argon2id$v%3D19$m%3D19456%2Ct%3D2%2Cp%3D1$GryY17H...18kP0; Max-Age=120;...
    ```
3. http://localhost:3000/todo  
   でCookieと以下のtodo JsonをPostする
    ```
    test_session_id = $argon2id$v%3D19$m%3D19456%2Ct%3D2%2Cp%3D1$GryY17H...18kP0;
    ```

    ```json
    {
        "text": "todo things"
    }
    ```

    レスポンスは以下のようになる
    ```json
    {
        "todo_id": 1,
        "user_id":  "5",
        "text": "todo things"
    }
    ```
## Caution⚠️
今回はAxumでのsession管理がどのようにできるか適当にテストするためであり、セキュリティの問題を多く抱えている。  
そのため、実際の開発ではセキュリティに関し、十分に考慮する必要がある。