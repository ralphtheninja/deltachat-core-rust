use std::borrow::Cow;
use std::fmt;

use crate::context::Context;
use crate::error::Error;

#[derive(Default, Debug)]
pub struct LoginParam {
    pub addr: String,
    pub mail_server: String,
    pub mail_user: String,
    pub mail_pw: String,
    pub mail_port: i32,
    pub send_server: String,
    pub send_user: String,
    pub send_pw: String,
    pub send_port: i32,
    pub server_flags: i32,
}

impl LoginParam {
    /// Create a new `LoginParam` with default values.
    pub fn new() -> Self {
        Default::default()
    }

    /// Read the login parameters from the database.
    pub fn from_database(context: &Context, prefix: impl AsRef<str>) -> Self {
        let prefix = prefix.as_ref();
        let sql = &context.sql;

        let key = format!("{}addr", prefix);
        let addr = sql
            .get_config(context, key)
            .unwrap_or_default()
            .trim()
            .to_string();

        let key = format!("{}mail_server", prefix);
        let mail_server = sql.get_config(context, key).unwrap_or_default();

        let key = format!("{}mail_port", prefix);
        let mail_port = sql.get_config_int(context, key).unwrap_or_default();

        let key = format!("{}mail_user", prefix);
        let mail_user = sql.get_config(context, key).unwrap_or_default();

        let key = format!("{}mail_pw", prefix);
        let mail_pw = sql.get_config(context, key).unwrap_or_default();

        let key = format!("{}send_server", prefix);
        let send_server = sql.get_config(context, key).unwrap_or_default();

        let key = format!("{}send_port", prefix);
        let send_port = sql.get_config_int(context, key).unwrap_or_default();

        let key = format!("{}send_user", prefix);
        let send_user = sql.get_config(context, key).unwrap_or_default();

        let key = format!("{}send_pw", prefix);
        let send_pw = sql.get_config(context, key).unwrap_or_default();

        let key = format!("{}server_flags", prefix);
        let server_flags = sql.get_config_int(context, key).unwrap_or_default();

        LoginParam {
            addr: addr.to_string(),
            mail_server,
            mail_user,
            mail_pw,
            mail_port,
            send_server,
            send_user,
            send_pw,
            send_port,
            server_flags,
        }
    }

    pub fn addr_str(&self) -> &str {
        self.addr.as_str()
    }

    /// Save this loginparam to the database.
    pub fn save_to_database(
        &self,
        context: &Context,
        prefix: impl AsRef<str>,
    ) -> Result<(), Error> {
        let prefix = prefix.as_ref();
        let sql = &context.sql;

        let key = format!("{}addr", prefix);
        sql.set_config(context, key, Some(&self.addr))?;

        let key = format!("{}mail_server", prefix);
        sql.set_config(context, key, Some(&self.mail_server))?;

        let key = format!("{}mail_port", prefix);
        sql.set_config_int(context, key, self.mail_port)?;

        let key = format!("{}mail_user", prefix);
        sql.set_config(context, key, Some(&self.mail_user))?;

        let key = format!("{}mail_pw", prefix);
        sql.set_config(context, key, Some(&self.mail_pw))?;

        let key = format!("{}send_server", prefix);
        sql.set_config(context, key, Some(&self.send_server))?;

        let key = format!("{}send_port", prefix);
        sql.set_config_int(context, key, self.send_port)?;

        let key = format!("{}send_user", prefix);
        sql.set_config(context, key, Some(&self.send_user))?;

        let key = format!("{}send_pw", prefix);
        sql.set_config(context, key, Some(&self.send_pw))?;

        let key = format!("{}server_flags", prefix);
        sql.set_config_int(context, key, self.server_flags)?;

        Ok(())
    }
}

impl fmt::Display for LoginParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unset = "0";
        let pw = "***";

        let flags_readable = get_readable_flags(self.server_flags);

        write!(
            f,
            "{} {}:{}:{}:{} {}:{}:{}:{} {}",
            unset_empty(&self.addr),
            unset_empty(&self.mail_user),
            if !self.mail_pw.is_empty() { pw } else { unset },
            unset_empty(&self.mail_server),
            self.mail_port,
            unset_empty(&self.send_user),
            if !self.send_pw.is_empty() { pw } else { unset },
            unset_empty(&self.send_server),
            self.send_port,
            flags_readable,
        )
    }
}

fn unset_empty(s: &String) -> Cow<String> {
    if s.is_empty() {
        Cow::Owned("unset".to_string())
    } else {
        Cow::Borrowed(s)
    }
}

fn get_readable_flags(flags: i32) -> String {
    let mut res = String::new();
    for bit in 0..31 {
        if 0 != flags & 1 << bit {
            let mut flag_added = 0;
            if 1 << bit == 0x2 {
                res += "OAUTH2 ";
                flag_added = 1;
            }
            if 1 << bit == 0x4 {
                res += "AUTH_NORMAL ";
                flag_added = 1;
            }
            if 1 << bit == 0x100 {
                res += "IMAP_STARTTLS ";
                flag_added = 1;
            }
            if 1 << bit == 0x200 {
                res += "IMAP_SSL ";
                flag_added = 1;
            }
            if 1 << bit == 0x400 {
                res += "IMAP_PLAIN ";
                flag_added = 1;
            }
            if 1 << bit == 0x10000 {
                res += "SMTP_STARTTLS ";
                flag_added = 1
            }
            if 1 << bit == 0x20000 {
                res += "SMTP_SSL ";
                flag_added = 1
            }
            if 1 << bit == 0x40000 {
                res += "SMTP_PLAIN ";
                flag_added = 1
            }
            if 0 == flag_added {
                res += &format!("{:#0x}", 1 << bit);
            }
        }
    }
    if res.is_empty() {
        res += "0";
    }

    res
}
