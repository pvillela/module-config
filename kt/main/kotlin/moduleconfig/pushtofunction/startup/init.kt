/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pushtofunction.startup

import tryout.moduleconfig.pushtofunction.config.AppCfgInfo
import tryout.moduleconfig.pushtofunction.config.getAppConfiguration
import tryout.moduleconfig.pushtofunction.fs.boot.fooSflBoot

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
