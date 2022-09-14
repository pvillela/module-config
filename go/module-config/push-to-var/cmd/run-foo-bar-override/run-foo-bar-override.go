/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package main

import (
	"github.com/pvillela/module-config/go/arch/util"
	"github.com/pvillela/module-config/go/module-config/push-to-var/fs"
)

func main() {
	fs.FooSflCfgSrcV = util.ThunkOf(fs.FooSflCfgInfo{X: "foo"})

	fs.BarBfCfgSrcV = func() fs.BarBfCfgInfo {
		return fs.BarBfCfgInfo{Z: 99}
	}

	fs.FooSfl()
}
