/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pullcfgwithoverride.run

import pullcfgwithoverride.fs.BazCfgInfo
import pullcfgwithoverride.fs.baz
import pullcfgwithoverride.fs.bazCfgSrc

fun main() {
    bazCfgSrc = { BazCfgInfo("baz") }

    baz()
}
