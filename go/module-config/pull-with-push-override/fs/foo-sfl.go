/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package fs

import (
	"fmt"
	"github.com/pvillela/moduleconfig/go/module-config/pull-with-push-override/config"
)

type FooSflCfgInfo struct {
	X string
}

type FooSflCfgSrc = config.CfgSrc[FooSflCfgInfo]

var FooSflCfgSrcV = config.MakeCfgSrc[FooSflCfgInfo](FooSflCfgAdapter)

func FooSfl() {
	fmt.Println(FooSflCfgSrcV().X)
	BarBf()
}
