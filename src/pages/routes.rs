/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use super::auth::routes::Auth;
use super::errors::routes::Errors;
use super::panel::routes::Panel;
pub const ROUTES: Routes = Routes::new();

pub struct Routes {
    pub home: &'static str,
    pub auth: Auth,
    pub panel: Panel,
    pub errors: Errors,
    pub about: &'static str,
    pub thanks: &'static str,
    pub donate: &'static str,
}

impl Routes {
    const fn new() -> Routes {
        let panel = Panel::new();
        let home = panel.home;
        Routes {
            auth: Auth::new(),
            panel,
            home,
            errors: Errors::new(),
            about: "/aboubt",
            thanks: "/thanks",
            donate: "/donat",
        }
    }
}
