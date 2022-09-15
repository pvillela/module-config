/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { bazCfgSrc, baz } from "../fs/baz";
import { thunkOf } from "../../arch/util/functions";

bazCfgSrc.set(thunkOf({ w: "baz" }))

baz()
