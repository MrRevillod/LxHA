use chrono::Datelike;

const LOGO_URL: &str = "https://i.ibb.co/C7B657H/logowm-03.pngD";
const DEFAULT_COLOR: &str = "#387EF2";

pub fn change_email_template(url: &str) -> String {
    format!(
        r#"
        <body>
            <table width="100%" bgcolor="{}">
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
                        <h2 style="color: black;">Email Change</h2>
                        <p style="color: black;">Thank you for joining LXHA! To change your email, please click on the following link:</p>
                        <p><a href="{}" style="color: #F5F5F5; background-color: {}; padding: 8px 16px; text-decoration: none; border-radius: 4px;">Change email</a></p>
                        <p style="color: black;">If you do not recognize this request, you can ignore this message.</p>
                        <p style="color: black;">Regards,</p>
                        <p style="color: black;">The LXHA Team</p>
                    </td>
                </tr>
                <tr>
                    <td>
                        <table width="100%" bgcolor="{}">
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
        DEFAULT_COLOR, LOGO_URL, url, DEFAULT_COLOR, DEFAULT_COLOR, chrono::Utc::now().year()
    )
}

pub fn reset_password_template(url: &str) -> String {
    format!(
        r#"
        <body>
            <table width="100%" bgcolor="{}">
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
                        <h2 style="color: black;">Password Reset</h2>
                        <p style="color: black;">We have received a request to reset your password. To do so, please click on the following link:</p>
                        <p><a href="{}" style="color: #F5F5F5; background-color: {}; padding: 8px 16px; text-decoration: none; border-radius: 4px;">Restore Password</a></p>
                        <p style="color: black;">If you do not recognize this request, you can ignore this message.</p>
                        <p style="color: black;">Regards,</p>
                        <p style="color: black;">The LXHA Team</p>
                    </td>
                </tr>
                <tr>
                    <td>
                        <table width="100%" bgcolor="{}">
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
        DEFAULT_COLOR, LOGO_URL, url, DEFAULT_COLOR, DEFAULT_COLOR, chrono::Utc::now().year()
    )
}

pub fn contact_from_admin_template(body: &str) -> String {
    format!(
        r#"
        <body>
            <table width="100%" bgcolor="{}">
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
                        <h2 style="color: black;">Message from lxHA Admin</h2>
                        <p style="color: black;">{}</p>
                    </td>
                </tr>
                <tr>
                    <td>
                        <table width="100%" bgcolor="{}">
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
        DEFAULT_COLOR, LOGO_URL, body, DEFAULT_COLOR, chrono::Utc::now().year()
    )
}

pub fn contact_from_user_template(body: &str, from_name: &str, from_email: &str) -> String {
    format!(
        r#"
        <body>
            <table width="100%" bgcolor="{}">
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
                        <h2 style="color: black;">Message from: </h2>
                        <p style="color: black;">User: {}</p>
                        <p style="color: black;">Email: {}</p>
                        <p style="color: black;">{}</p>
                    </td>
                </tr>
                <tr>
                    <td>
                        <table width="100%" bgcolor="{}">
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
        DEFAULT_COLOR, LOGO_URL, from_name, from_email, body, DEFAULT_COLOR, chrono::Utc::now().year()
    )
}

pub fn new_account_message_template(user_email: &str, user_password: &str) -> String {
    format!(
        r#"
        <body>
            <table width="100%" bgcolor="{}">
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
                        <h2 style="color: black;">Welcome to lxHA</h2>
                        <p style="color: black;">Your user account has been created successfully, your credentials are:</p>
                        <p style="color: black;">Email: {}</p>
                        <p style="color: black;">Password: {}</p>
                        <p style="color: black;">It is your responsibility not to share your credentials.</p>
                    </td>
                </tr>
                <tr>
                    <td>
                        <table width="100%" bgcolor="{}">
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
        DEFAULT_COLOR, LOGO_URL, user_email, user_password, DEFAULT_COLOR, chrono::Utc::now().year()
    )
}

pub fn new_instance_message_template(password: &str) -> String {
    format!(
        r#"
        <body>
            <table width="100%" bgcolor="{}">
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
                        <h2 style="color: black;">Welcome to lxHA</h2>
                        <p style="color: black;">Your user account has been created successfully, your credentials are:</p>
                        <p style="color: black;">Password: {}</p>
                        <p style="color: black;">It is your responsibility not to share your credentials.</p>
                    </td>
                </tr>
                <tr>
                    <td>
                        <table width="100%" bgcolor="{}">
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
        DEFAULT_COLOR, LOGO_URL, password, DEFAULT_COLOR, chrono::Utc::now().year()
    )
}