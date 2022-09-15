/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package main

import (
	"github.com/pvillela/module-config/go/arch/util"
	fs2 "github.com/pvillela/module-config/go/pull-with-push-override/fs"
)

func main() {
	fs2.FooSflCfgSrcV = util.ThunkOf(fs2.FooSflCfgInfo{X: "foo"})

	fs2.BarBfCfgSrcV = func() fs2.BarBfCfgInfo {
		return fs2.BarBfCfgInfo{Z: 99}
	}

	fs2.FooSfl()
}
