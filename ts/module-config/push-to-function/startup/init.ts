/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { getAppConfiguration as c } from "../config/app-cfg";
import { fooSflBoot } from "../fs/boot/foo-sfl-boot";

export const fooSfl = fooSflBoot(c);
