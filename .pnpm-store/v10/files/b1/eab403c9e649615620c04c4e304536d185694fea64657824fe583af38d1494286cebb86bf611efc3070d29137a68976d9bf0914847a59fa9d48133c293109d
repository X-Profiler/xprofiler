// src/index.ts
import * as fs from "fs";
import globRex from "globrex";
import { resolve as resolve3 } from "path";
import { inspect } from "util";
import { normalizePath as normalizePath2, searchForWorkspaceRoot } from "vite";

// src/mappings.ts
import { resolve } from "path";
function resolvePathMappings(paths, base) {
  const sortedPatterns = Object.keys(paths).sort(
    (a, b) => getPrefixLength(b) - getPrefixLength(a)
  );
  const resolved = [];
  for (let pattern of sortedPatterns) {
    const relativePaths = paths[pattern];
    pattern = escapeStringRegexp(pattern).replace(/\*/g, "(.+)");
    resolved.push({
      pattern: new RegExp("^" + pattern + "$"),
      paths: relativePaths.map((relativePath) => resolve(base, relativePath))
    });
  }
  return resolved;
}
function getPrefixLength(pattern) {
  const prefixLength = pattern.indexOf("*");
  return pattern.substr(0, prefixLength).length;
}
function escapeStringRegexp(string) {
  return string.replace(/[|\\{}()[\]^$+?.]/g, "\\$&").replace(/-/g, "\\x2d");
}

// src/path.ts
import * as os from "os";
import * as path from "path";
import { normalizePath } from "vite";
import { dirname } from "path";
var isWindows = os.platform() == "win32";
var resolve2 = isWindows ? (...paths) => normalizePath(path.win32.resolve(...paths)) : path.posix.resolve;
var isAbsolute = isWindows ? path.win32.isAbsolute : path.posix.isAbsolute;
var join = path.posix.join;
var relative = path.posix.relative;
var basename = path.posix.basename;

// src/debug.ts
import createDebug from "debug";
var debug = createDebug("vite-tsconfig-paths");
var debugResolve = createDebug("vite-tsconfig-paths:resolve");
if (process.env.TEST) {
  createDebug.log = console.log.bind(console);
}

// src/index.ts
var notApplicable = [void 0, false];
var notFound = [void 0, true];
var src_default = (opts = {}) => {
  let resolversByDir;
  return {
    name: "vite-tsconfig-paths",
    enforce: "pre",
    async configResolved(config) {
      let projectRoot = config.root;
      let workspaceRoot;
      let { root } = opts;
      if (root) {
        root = resolve3(projectRoot, root);
      } else {
        workspaceRoot = searchForWorkspaceRoot(projectRoot);
      }
      debug("options.root   ==", root);
      debug("project root   ==", projectRoot);
      debug("workspace root ==", workspaceRoot);
      if (root) {
        projectRoot = root;
        workspaceRoot = root;
      }
      const tsconfck = await import("tsconfck");
      const projects = opts.projects ? opts.projects.map((file) => {
        if (!file.endsWith(".json")) {
          file = join(file, "tsconfig.json");
        }
        return resolve3(projectRoot, file);
      }) : await tsconfck.findAll(workspaceRoot, {
        configNames: opts.configNames || ["tsconfig.json", "jsconfig.json"],
        skip(dir) {
          if (dir === ".git" || dir === "node_modules") {
            return true;
          }
          if (typeof opts.skip === "function") {
            return opts.skip(dir);
          }
          return false;
        }
      });
      debug("projects:", projects);
      let hasTypeScriptDep = false;
      if (opts.parseNative) {
        try {
          const pkgJson = fs.readFileSync(
            join(workspaceRoot, "package.json"),
            "utf8"
          );
          const pkg = JSON.parse(pkgJson);
          const deps = { ...pkg.dependencies, ...pkg.devDependencies };
          hasTypeScriptDep = "typescript" in deps;
        } catch (e) {
          if (e.code != "ENOENT") {
            throw e;
          }
        }
      }
      let firstError;
      const parseOptions = {
        cache: new tsconfck.TSConfckCache()
      };
      const parsedProjects = new Set(
        await Promise.all(
          projects.map((tsconfigFile) => {
            if (tsconfigFile === null) {
              debug("tsconfig file not found:", tsconfigFile);
              return null;
            }
            return (hasTypeScriptDep ? tsconfck.parseNative(tsconfigFile, parseOptions) : tsconfck.parse(tsconfigFile, parseOptions)).catch((error) => {
              if (opts.ignoreConfigErrors) {
                debug("tsconfig file caused a parsing error:", tsconfigFile);
              } else {
                config.logger.error(
                  '[tsconfig-paths] An error occurred while parsing "' + tsconfigFile + '". See below for details.' + (firstError ? "" : " To disable this message, set the `ignoreConfigErrors` option to true."),
                  { error }
                );
                if (config.logger.hasErrorLogged(error)) {
                  console.error(error);
                }
                firstError = error;
              }
              return null;
            });
          })
        )
      );
      resolversByDir = {};
      parsedProjects.forEach((project) => {
        if (!project) {
          return;
        }
        if (project.referenced) {
          project.referenced.forEach((projectRef) => {
            parsedProjects.add(projectRef);
          });
          parsedProjects.delete(project);
          parsedProjects.add(project);
          project.referenced = void 0;
        } else {
          const resolver = createResolver(project);
          if (resolver) {
            const projectDir = normalizePath2(dirname(project.tsconfigFile));
            const resolvers = resolversByDir[projectDir] || (resolversByDir[projectDir] = []);
            resolvers.push(resolver);
          }
        }
      });
    },
    async resolveId(id, importer, options) {
      if (debugResolve.enabled) {
        debugResolve("resolving:", { id, importer });
      }
      if (!importer) {
        debugResolve("importer is empty or undefined. skipping...");
        return;
      }
      if (relativeImportRE.test(id)) {
        debugResolve("id is a relative import. skipping...");
        return;
      }
      if (isAbsolute(id)) {
        debugResolve("id is an absolute path. skipping...");
        return;
      }
      if (id.includes("\0")) {
        debugResolve("id is a virtual module. skipping...");
        return;
      }
      const resolveOptions = { ...options, skipSelf: true };
      const viteResolve = async (id2, importer2) => {
        var _a;
        return (_a = await this.resolve(id2, importer2, resolveOptions)) == null ? void 0 : _a.id;
      };
      let prevProjectDir;
      let projectDir = normalizePath2(dirname(importer));
      loop:
        while (projectDir && projectDir != prevProjectDir) {
          const resolvers = resolversByDir[projectDir];
          if (resolvers) {
            for (const resolve4 of resolvers) {
              const [resolved, matched] = await resolve4(viteResolve, id, importer);
              if (resolved) {
                return resolved;
              }
              if (matched) {
                break loop;
              }
            }
          }
          prevProjectDir = projectDir;
          projectDir = dirname(prevProjectDir);
        }
    }
  };
  function resolvePathsRootDir(project) {
    var _a, _b, _c, _d;
    if ("result" in project) {
      return (_b = (_a = project.result.options) == null ? void 0 : _a.pathsBasePath) != null ? _b : dirname(project.tsconfigFile);
    }
    const baseUrl = (_c = project.tsconfig.compilerOptions) == null ? void 0 : _c.baseUrl;
    if (baseUrl) {
      return baseUrl;
    }
    const projectWithPaths = (_d = project.extended) == null ? void 0 : _d.find(
      (p) => {
        var _a2;
        return (_a2 = p.tsconfig.compilerOptions) == null ? void 0 : _a2.paths;
      }
    );
    return dirname((projectWithPaths != null ? projectWithPaths : project).tsconfigFile);
  }
  function createResolver(project) {
    var _a, _b, _c, _d;
    const configPath = normalizePath2(project.tsconfigFile);
    const config = project.tsconfig;
    debug("config loaded:", inspect({ configPath, config }, false, 10, true));
    if (((_a = config.files) == null ? void 0 : _a.length) == 0 && !((_b = config.include) == null ? void 0 : _b.length)) {
      debug(
        `[!] skipping "${configPath}" as no files can be matched since "files" is empty and "include" is missing or empty`
      );
      return null;
    }
    const options = config.compilerOptions || {};
    const { baseUrl, paths } = options;
    if (!baseUrl && !paths) {
      debug(`[!] missing baseUrl and paths: "${configPath}"`);
      return null;
    }
    const resolveWithBaseUrl = baseUrl ? (viteResolve, id, importer) => {
      const absoluteId = join(baseUrl, id);
      debugResolve("trying with baseUrl:", absoluteId);
      return viteResolve(absoluteId, importer);
    } : void 0;
    let resolveId;
    if (paths) {
      const pathsRootDir = resolvePathsRootDir(project);
      const pathMappings = resolvePathMappings(paths, pathsRootDir);
      const resolveWithPaths = async (viteResolve, id, importer) => {
        for (const mapping of pathMappings) {
          const match = id.match(mapping.pattern);
          if (!match) {
            continue;
          }
          for (let pathTemplate of mapping.paths) {
            let starCount = 0;
            const mappedId = pathTemplate.replace(/\*/g, () => {
              const matchIndex = Math.min(++starCount, match.length - 1);
              return match[matchIndex];
            });
            debugResolve("found match, trying to resolve:", mappedId);
            const resolved = await viteResolve(mappedId, importer);
            if (resolved) {
              return resolved;
            }
          }
        }
      };
      if (resolveWithBaseUrl) {
        resolveId = (viteResolve, id, importer) => resolveWithPaths(viteResolve, id, importer).then((resolved) => {
          return resolved != null ? resolved : resolveWithBaseUrl(viteResolve, id, importer);
        });
      } else {
        resolveId = resolveWithPaths;
      }
    } else {
      resolveId = resolveWithBaseUrl;
    }
    const configDir = dirname(configPath);
    let { outDir } = options;
    if (outDir && isAbsolute(outDir)) {
      outDir = relative(configDir, outDir);
    }
    const isIncludedRelative = getIncluder(
      (_c = config.include) == null ? void 0 : _c.map((p) => ensureRelative(configDir, p)),
      (_d = config.exclude) == null ? void 0 : _d.map((p) => ensureRelative(configDir, p)),
      outDir
    );
    const importerExtRE = opts.loose ? /./ : options.allowJs || basename(configPath).startsWith("jsconfig.") ? jsLikeRE : /\.[mc]?tsx?$/;
    const resolutionCache = /* @__PURE__ */ new Map();
    return async (viteResolve, id, importer) => {
      var _a2;
      importer = normalizePath2(importer);
      const importerFile = importer.replace(/[#?].+$/, "");
      if (!importerExtRE.test(importerFile)) {
        debugResolve("importer has unsupported extension. skipping...");
        return notApplicable;
      }
      const relativeImporterFile = relative(configDir, importerFile);
      if (!isIncludedRelative(relativeImporterFile)) {
        debugResolve("importer is not included. skipping...");
        return notApplicable;
      }
      const suffix = (_a2 = /\?.+$/.exec(id)) == null ? void 0 : _a2[0];
      if (suffix) {
        id = id.slice(0, -suffix.length);
      }
      let resolvedId = resolutionCache.get(id);
      if (!resolvedId) {
        resolvedId = await resolveId(viteResolve, id, importer);
        if (!resolvedId) {
          return notFound;
        }
        resolutionCache.set(id, resolvedId);
        if (debugResolve.enabled) {
          debugResolve("resolved without error:", {
            id,
            importer,
            resolvedId,
            configPath
          });
        }
      }
      if (suffix) {
        resolvedId += suffix;
      }
      return [resolvedId, true];
    };
  }
};
var jsLikeRE = /\.(vue|svelte|mdx|[mc]?[jt]sx?)$/;
var relativeImportRE = /^\.\.?(\/|$)/;
var defaultInclude = ["**/*"];
var defaultExclude = [
  "**/node_modules",
  "**/bower_components",
  "**/jspm_packages"
];
function getIncluder(includePaths = defaultInclude, excludePaths = defaultExclude, outDir) {
  if (outDir) {
    excludePaths = excludePaths.concat(outDir);
  }
  if (includePaths.length || excludePaths.length) {
    const includers = [];
    const excluders = [];
    includePaths.forEach(addCompiledGlob, includers);
    excludePaths.forEach(addCompiledGlob, excluders);
    debug(`compiled globs:`, { includers, excluders });
    return (path2) => {
      path2 = path2.replace(/\?.+$/, "");
      if (!relativeImportRE.test(path2)) {
        path2 = "./" + path2;
      }
      const test = (glob) => glob.test(path2);
      return includers.some(test) && !excluders.some(test);
    };
  }
  return () => true;
}
function addCompiledGlob(glob) {
  const endsWithGlob = glob.split("/").pop().includes("*");
  const relativeGlob = relativeImportRE.test(glob) ? glob : "./" + glob;
  if (endsWithGlob) {
    this.push(compileGlob(relativeGlob));
  } else {
    this.push(compileGlob(relativeGlob + "/**"));
    if (/\.\w+$/.test(glob)) {
      this.push(compileGlob(relativeGlob));
    }
  }
}
function compileGlob(glob) {
  return globRex(glob, {
    extended: true,
    globstar: true
  }).regex;
}
function ensureRelative(dir, path2) {
  return isAbsolute(path2) ? relative(dir, path2) : path2;
}
export {
  src_default as default
};
//# sourceMappingURL=index.js.map