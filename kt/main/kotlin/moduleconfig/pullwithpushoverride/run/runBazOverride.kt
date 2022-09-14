/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pullwithpushoverride.run

import tryout.moduleconfig.pullwithpushoverride.fs.BazCfgInfo
import tryout.moduleconfig.pullwithpushoverride.fs.baz
import tryout.moduleconfig.pullwithpushoverride.fs.bazCfgSrc

fun main() {
    bazCfgSrc = { BazCfgInfo("baz") }

    baz()
}
