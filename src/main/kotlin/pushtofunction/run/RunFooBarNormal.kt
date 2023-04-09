/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pushtofunction.run

import pushtofunction.config.AppCfgInfo
import pushtofunction.config.appCfgInfo
import pushtofunction.startup.fooSfl
import pushtofunction.startup.fooSfl1

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
