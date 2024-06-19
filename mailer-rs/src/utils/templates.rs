use chrono::Datelike;

const LOGO_URL: &str = "https://i.ibb.co/C7B657H/logowm-03.pngD";

pub fn change_email_template(url: &str) -> String {
    format!(
        r#"
        <body>
            <table width="100%" bgcolor="0x387EF2">
                <tr>
                    <td>
                        <h1 style="color: white; padding: 0px;">LXHA</h1>
                    </td>
                    <td style="width: 70%;"></td>
                    <td >
                        <img style="max-width: 60px;" src="{}" alt="logo">
                    </td>
                </tr>
            </table>

            <table width="100%">
                <tr>
                    <td>
                        <h2 style="color: black;">Cambio de Correo Electrónico</h2>
                        <p style="color: black;">¡Gracias por unirte a LXHA! Para cambiar tu correo electrónico, por favor haz clic en el siguiente enlace:</p>
                        <p><a href="{}" style="color: #F5F5F5; background-color: #387EF2; padding: 8px 16px; text-decoration: none; border-radius: 4px;">Cambiar correo electrónico</a></p>
                        <p style="color: black;">Si no reconoces esta solicitud, puedes ignorar este mensaje.</p>
                        <p style="color: black;">Atentamente,</p>
                        <p style="color: black;">El equipo de LXHA</p>
                    </td>
                </tr>
                <tr>
                    <td>
                        <table width="100%" bgcolor="0x387EF2">
                            <tr>
                                <td>
                                    <p style="color: white; text-align: center; padding: 10px;">© {} LXHA</p>
                                </td>
                            </tr>
                        </table>
                    </td>
                </tr>
            </table>
        </body>
    "#,
        LOGO_URL, url, chrono::Utc::now().year()
    )
}

pub fn reset_password_template(url: &str) -> String {
    format!(
        r#"
        <body>
            <table width="100%" bgcolor="0x387EF2">
                <tr>
                    <td>
                        <h1 style="color: white; padding: 0px;">LXHA</h1>
                    </td>
                    <td style="width: 70%;"></td>
                    <td >
                        <img style="max-width: 60px;" src="{}" alt="logo">
                    </td>
                </tr>
            </table>

            <table width="100%">
                <tr>
                    <td>
                        <h2 style="color: black;">Restauración de Contraseña</h2>
                        <p style="color: black;">Hemos recibido una solicitud para restaurar tu contraseña. Para hacerlo, por favor haz clic en el siguiente enlace:</p>
                        <p><a href="{}" style="color: #F5F5F5; background-color: #387EF2; padding: 8px 16px; text-decoration: none; border-radius: 4px;">Restaurar Contraseña</a></p>
                        <p style="color: black;">Si no reconoces esta solicitud, puedes ignorar este mensaje.</p>
                        <p style="color: black;">Atentamente,</p>
                        <p style="color: black;">El equipo de LXHA</p>
                    </td>
                </tr>
                <tr>
                    <td>
                        <table width="100%" bgcolor="0x387EF2">
                            <tr>
                                <td>
                                    <p style="color: white; text-align: center; padding: 10px;">© {} LXHA</p>
                                </td>
                            </tr>
                        </table>
                    </td>
                </tr>
            </table>
        </body>
    "#,
        LOGO_URL, url, chrono::Utc::now().year()
    )
}

pub fn contact_from_admin_template(body: &str) -> String {
    format!(
        r#"
        <div>
            New message from Admin
        </div>
        </br>
        <div>
            {}
        </div>
    "#,
        body
    )
}

pub fn contact_from_user_template(body: &str, from_name: &str, from_email: &str) -> String {
    format!(
        r#"
            <div>
                New message from user {} - {}
            </div>
            </br>
            <div>
                {}
            </div>
    "#,
        from_name, from_email, body
    )
}

pub fn new_account_message_template(user_email: &str, user_password: &str) -> String {
    format!(
        r#"
            <div>{}</div><br/><div>{}</div>
    "#,
        user_email, user_password
    )
}