/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pushtofunction.run

import tryout.moduleconfig.pushtofunction.config.AppCfgInfo
import tryout.moduleconfig.pushtofunction.config.appCfgInfo
import tryout.moduleconfig.pushtofunction.startup.fooSfl
import tryout.moduleconfig.pushtofunction.startup.fooSfl1

fun main() {
    fooSfl()
    fooSfl1()

    // Change of app config properties at runtime
    appCfgInfo.set(
        AppCfgInfo(
            "YYY",
            84
        )
    )
    fooSfl()
}
