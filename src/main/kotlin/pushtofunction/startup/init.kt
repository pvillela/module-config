/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pushtofunction.startup

import pushtofunction.config.AppCfgInfo
import pushtofunction.config.getAppConfiguration
import pushtofunction.fs.boot.fooSflBoot

val fooSfl = fooSflBoot(::getAppConfiguration)

val fooSfl1 = run {
    val appCfgSrc1 = {
        AppCfgInfo(
            "foo",
            99
        )
    }
    fooSflBoot(appCfgSrc1)
}
