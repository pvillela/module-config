/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package fs

import (
	"fmt"
	"github.com/pvillela/module-config/go/pull-with-push-override/config"
)

type BarBfCfgInfo struct {
	Z int
}

type BarBfCfgSrc = config.CfgSrc[BarBfCfgInfo]

var BarBfCfgSrcV = config.MakeCfgSrc[BarBfCfgInfo](BarBfCfgAdapter)

func BarBf() {
	fmt.Println(BarBfCfgSrcV().Z)
}
