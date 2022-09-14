/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package fs

import (
	"fmt"
	"github.com/pvillela/module-config/go/module-config/push-to-var/fwk"
)

type FooSflCfgInfo struct {
	X string
}

var FooSflCfgSrcV = fwk.NilCfgSrc[FooSflCfgInfo]

func FooSfl() {
	fmt.Println(FooSflCfgSrcV().X)
	BarBf()
}
