/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pushtovar.fwk

class CfgSrc<T>(
    private var src: () -> T = ::nilCfgSrc
) {
    fun set(src: () -> T) {
        this.src = src
    }

    operator fun invoke(): T {
        return src()
    }
}

private fun <T>nilCfgSrc(): T {
    throw ConfigurationException("Module used before being initialized")
}

typealias CfgSrcAdapter<S, T> = (S) -> T

class CfgSrcAdaptation<S, T>(
    private val targetSrc: CfgSrc<T>,
    private val adapter: CfgSrcAdapter<S, T>
) {
    fun setOrigin(originSrc: () -> S) {
        this.targetSrc.set { this.adapter(originSrc()) }
    }
}
