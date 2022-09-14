/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package fs

import (
	"fmt"
	"github.com/pvillela/moduleconfig/go/module-config/push-to-var/fwk"
)

type BarBfCfgInfo struct {
	Z int
}

type BarBfCfgSrc = fwk.CfgSrc[BarBfCfgInfo]

var BarBfCfgSrcV = fwk.NilCfgSrc[BarBfCfgInfo]

func BarBf() {
	fmt.Println(BarBfCfgSrcV().Z)
}
