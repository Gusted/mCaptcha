/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
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

import validateDescription from "./validateDescription";
import {getAddForm, fillDescription} from "../setupTests";
import {mockAlert} from "../../../../../../setUpTests";

import setup from "../../../../../../components/error/setUpTests";

mockAlert();

document.body.innerHTML = getAddForm();

const emptyErr = "can't be empty";

it("validateDescription workds", () => {
  document.querySelector("body").appendChild(setup());
  try {
    const event = new Event("submit");
    validateDescription(event);
  } catch (e) {
    expect(e.message).toContain(emptyErr);
  }

  // fill and validate
  fillDescription("testing");
  const event = new Event("submit");
  validateDescription(event);
});
