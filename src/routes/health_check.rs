use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body(String::from(
        r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Health Check Success</title>
                <style>
                    body {
                        font-family: Arial, sans-serif;
                        background-color: #f5f5f5;
                        margin: 0;
                        padding: 0;
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        height: 100vh;
                    }

                    .container {
                        text-align: center;
                        padding: 50px;
                        border-radius: 10px;
                        background-color: #fff;
                        box-shadow: 0 0 20px rgba(0, 0, 0, 0.1);
                    }

                    h1 {
                        color: #4CAF50;
                        margin-bottom: 20px;
                    }

                    p {
                        color: #333;
                        font-size: 18px;
                        margin-bottom: 30px;
                    }

                    .success-icon {
                        color: #4CAF50;
                        font-size: 60px;
                        margin-bottom: 30px;
                    }

                    .button {
                        padding: 10px 20px;
                        background-color: #4CAF50;
                        color: #fff;
                        border: none;
                        border-radius: 5px;
                        font-size: 16px;
                        cursor: pointer;
                        transition: background-color 0.3s ease;
                    }

                    .button:hover {
                        background-color: #45a049;
                    }
                </style>
            </head>
            <body>
                <div class="container">
                    <i class="fas fa-check-circle success-icon"></i>
                    <h1>Health Check Successful!</h1>
                    <p>Congratulations! Your system has passed the health check without any issues.</p>
                    <button class="button" onclick="window.location.href = 'https://example.com'">Return Home</button>
                </div>
            </body>
            </html>
        "#,
    ))
}
