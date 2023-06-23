/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'spx_rust_backend.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./spx_rust_backend.android-arm64.node')
          } else {
            nativeBinding = require('spx_rust_backend-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'spx_rust_backend.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./spx_rust_backend.android-arm-eabi.node')
          } else {
            nativeBinding = require('spx_rust_backend-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'spx_rust_backend.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./spx_rust_backend.win32-x64-msvc.node')
          } else {
            nativeBinding = require('spx_rust_backend-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'spx_rust_backend.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./spx_rust_backend.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('spx_rust_backend-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'spx_rust_backend.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./spx_rust_backend.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('spx_rust_backend-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'spx_rust_backend.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./spx_rust_backend.darwin-universal.node')
      } else {
        nativeBinding = require('spx_rust_backend-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'spx_rust_backend.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./spx_rust_backend.darwin-x64.node')
          } else {
            nativeBinding = require('spx_rust_backend-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'spx_rust_backend.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./spx_rust_backend.darwin-arm64.node')
          } else {
            nativeBinding = require('spx_rust_backend-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'spx_rust_backend.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./spx_rust_backend.freebsd-x64.node')
      } else {
        nativeBinding = require('spx_rust_backend-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'spx_rust_backend.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./spx_rust_backend.linux-x64-musl.node')
            } else {
              nativeBinding = require('spx_rust_backend-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'spx_rust_backend.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./spx_rust_backend.linux-x64-gnu.node')
            } else {
              nativeBinding = require('spx_rust_backend-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'spx_rust_backend.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./spx_rust_backend.linux-arm64-musl.node')
            } else {
              nativeBinding = require('spx_rust_backend-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'spx_rust_backend.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./spx_rust_backend.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('spx_rust_backend-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(
          join(__dirname, 'spx_rust_backend.linux-arm-gnueabihf.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./spx_rust_backend.linux-arm-gnueabihf.node')
          } else {
            nativeBinding = require('spx_rust_backend-linux-arm-gnueabihf')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { sma, ema, macd, sd, bollingerBands, kdj, tr, atr, kc, donchianMidline, highest, lowest, linearRegression, mean, ttmSqueeze, testRun, getPrices, getEntries } = nativeBinding

module.exports.sma = sma
module.exports.ema = ema
module.exports.macd = macd
module.exports.sd = sd
module.exports.bollingerBands = bollingerBands
module.exports.kdj = kdj
module.exports.tr = tr
module.exports.atr = atr
module.exports.kc = kc
module.exports.donchianMidline = donchianMidline
module.exports.highest = highest
module.exports.lowest = lowest
module.exports.linearRegression = linearRegression
module.exports.mean = mean
module.exports.ttmSqueeze = ttmSqueeze
module.exports.testRun = testRun
module.exports.getPrices = getPrices
module.exports.getEntries = getEntries
