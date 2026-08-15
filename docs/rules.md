---
icon: lucide/list-checks
---

# Rules Reference

## Overview
<!-- GENERATED: Rule Table -->

| Check ID | Category | Description | Default Severity | Auto-fix |
| -------- | -------- | ----------- | ---------------- | -------- |
| [`NOSEMI`](rules/nosemi.md) | Formatting | Statement without trailing semicolon may produce unintended console output | Info | Yes |
| [`IFBDUP`](rules/bugs.md#ifbdup) | Bugs | Duplicate if-branch bodies | Error | No |
| [`IFCDUP`](rules/bugs.md#ifcdup) | Bugs | Duplicate if-branch conditions | Error | No |
| [`CTRUE`](rules/bugs.md#ctrue) | Bugs | Condition is always true (`if true`, `while 1`) | Error | No |
| [`CFALSE`](rules/bugs.md#cfalse) | Bugs | Condition is always false (`if false`, `while 0`) | Error | No |
| [`SHOCIRT`](rules/bugs.md#shocirt) | Bugs | Short-circuit `&&` with non-scalar LHS | Error | No |
| [`SHOCIRF`](rules/bugs.md#shocirf) | Bugs | Short-circuit `\\|\\|` with non-scalar LHS | Error | No |
| [`DEBUGFUN`](rules/bugs.md#debugfun) | Bugs | Debug function in code (keyboard, dbstop, etc.) | Error | No |
| [`INCR`](rules/bugs.md#incr) | Bugs | Suspicious self-increment `x = x + 1` | Error | No |
| [`DECR`](rules/bugs.md#decr) | Bugs | Suspicious self-decrement `x = x - 1` | Error | No |
| [`CMDAND`](rules/bugs.md#cmdand) | Bugs | `&` used where `&&` intended (boolean context) | Error | Yes |
| [`CMDOR`](rules/bugs.md#cmdor) | Bugs | `\\|` used where `\\|\\|` intended (boolean context) | Error | Yes |
| [`RHSFN`](rules/bugs.md#rhsfn) | Bugs | Function name used on RHS without `@` | Error | Yes |
| [`FNAN`](rules/bugs.md#fnan) | Bugs | Comparison with NaN (use `isnan` instead) | Error | Yes |
| [`LOGEMP`](rules/bugs.md#logemp) | Bugs | `length(x) == 0` instead of `isempty(x)` | Error | Yes |
| [`STCUL`](rules/bugs.md#stcul) | Bugs | `strcmpi` with same-case arguments | Error | No |
| [`LBODUP`](rules/bugs.md#lbodup) | Bugs | Duplicate case values in switch | Error | No |
| [`FUNFUN`](rules/bugs.md#funfun) | Bugs | Passing function name as string instead of handle | Error | No |
| [`DEFSIZE`](rules/bugs.md#defsize) | Bugs | `size(x) == [m n]` instead of `isequal(size(x), [m n])` | Error | Yes |
| [`VARARG`](rules/bugs.md#vararg) | Bugs | Misuse of varargin/varargout | Error | No |
| [`STRCMPCSTR`](rules/bugs.md#strcmpcstr) | Bugs | `strcmp` with single-char comparison | Error | No |
| [`ASSRT`](rules/bugs.md#assrt) | Bugs | `assert` with constant true condition | Error | No |
| [`BDSCA2`](rules/bugs.md#bdsca2) | Bugs | Suspicious scalar/array operation | Error | No |
| [`NOPRC`](rules/bugs.md#noprc) | Bugs | No `otherwise` in switch | Error | No |
| [`MOCUP`](rules/bugs.md#mocup) | Bugs | Operator precedence issue | Error | No |
| [`MDUPC`](rules/bugs.md#mdupc) | Bugs | Duplicate case in switch | Error | No |
| [`MNANC`](rules/bugs.md#mnanc) | Bugs | Comparison with NaN (alternate form) | Error | Yes |
| [`MULCC`](rules/bugs.md#mulcc) | Bugs | Multiple conditions could be simplified | Error | Yes |
| [`MEXCEP`](rules/bugs.md#mexcep) | Bugs | Catch without identifier | Error | No |
| [`PFUIXE`](rules/bugs.md#pfuixe) | Bugs | Parfor index used in eval | Error | No |
| [`PFBFN`](rules/bugs.md#pfbfn) | Bugs | Builtin function in parfor | Error | No |
| [`PFWHOS`](rules/bugs.md#pfwhos) | Bugs | who/whos in parfor | Error | No |
| [`PFTUSE`](rules/bugs.md#pftuse) | Bugs | Temporary variable misuse in parfor | Error | No |
| [`PFRNC`](rules/bugs.md#pfrnc) | Bugs | Reduction not consistent in parfor | Error | No |
| [`FWPARF`](rules/bugs.md#fwparf) | Bugs | For loop could be parfor | Error | No |
| [`PFTRIV`](rules/bugs.md#pftriv) | Bugs | Parfor could be for | Error | No |
| [`AGROW`](rules/performance.md) | Performance | Variable appears to change size on every loop iteration; consider preallocating | Info | No |
| [`SAGROW`](rules/performance.md) | Performance | Sliced variable appears to grow inside a loop | Info | No |
| [`PFBNS`](rules/performance.md) | Performance | Prefer broadcasting syntax over bsxfun calls | Info | No |
| [`RGXP1`](rules/performance.md) | Performance | Use regexp with output arguments instead of regexpi when case is known | Info | No |
| [`RGXPI`](rules/performance.md) | Performance | Use regexpi output argument form | Info | No |
| [`TRIM1`](rules/performance.md) | Performance | Use strtrim instead of deblank for trimming leading/trailing whitespace | Info | No |
| [`TRIM2`](rules/performance.md) | Performance | Prefer strtrim over deblank | Info | No |
| [`STTOK`](rules/performance.md) | Performance | Prefer split/strsplit over strtok | Info | No |
| [`STNCI`](rules/performance.md) | Performance | Use startsWith instead of comparing the first characters of a string | Info | No |
| [`STCCS`](rules/performance.md) | Performance | Use strcmp for character-vector comparison | Info | No |
| [`FNDSB`](rules/performance.md) | Performance | Prefer find(x > 0, 1) over find(x, 1) style patterns | Info | No |
| [`SFLD`](rules/performance.md) | Performance | Use dynamic field names instead of setfield | Info | No |
| [`GFLD`](rules/performance.md) | Performance | Use dynamic field names instead of getfield | Info | No |
| [`CCAT`](rules/performance.md) | Performance | Concatenate cell arrays using [] instead of extracting and reconstructing | Info | No |
| [`CCAT1`](rules/performance.md) | Performance | {A{I}} can usually be replaced by A(I) or A(I)' | Info | No |
| [`ISMT`](rules/performance.md) | Performance | Use ismatrix instead of comparing ndims to 2 | Info | No |
| [`ISCL`](rules/performance.md) | Performance | Use isscalar instead of numel(x)==1 | Info | No |
| [`ST2NM`](rules/performance.md) | Performance | Prefer str2double over str2num | Info | No |
| [`FLPST`](rules/performance.md) | Performance | Prefer flip/rot90 over flipud/fliplr where equivalent | Info | No |
| [`MXFND`](rules/performance.md) | Performance | Use max with a single output when only the value is needed | Info | No |
| [`EFIND`](rules/performance.md) | Performance | Use the faster find form for simple conditions | Info | No |
| [`EXIST`](rules/performance.md) | Performance | Use isfile/isfolder instead of exist | Info | No |
| [`UDIM`](rules/performance.md) | Performance | Use numel instead of size for a single dimension when the array is 1-D | Info | No |
| [`FREAD`](rules/performance.md) | Performance | Use fread with fewer output arguments when possible | Info | No |
| [`N2UNI`](rules/performance.md) | Performance | Use unique instead of manual sort+diff patterns | Info | No |
| [`TNMLP`](rules/performance.md) | Performance | Prefer strlength over numel for strings | Info | No |
| [`MINV`](rules/performance.md) | Performance | Use A\b instead of inv(A)*b | Info | No |
| [`LAXES`](rules/performance.md) | Performance | Prefer axes() with explicit arguments | Info | No |
| [`MMTC`](rules/performance.md) | Performance | Use mtimes/mtimesc scalar-matrix shortcuts | Info | No |
| [`MRPBW`](rules/performance.md) | Performance | Prefer repmat-avoiding broadcasting | Info | No |
| [`SPRIX`](rules/performance.md) | Performance | Use sparse indexing forms | Info | No |
| [`TRSRT`](rules/performance.md) | Performance | Use issorted instead of manual sort comparisons | Info | No |
| [`GRIDD`](rules/performance.md) | Performance | Prefer ndgrid over meshgrid where appropriate | Info | No |
| [`AND2`](rules/performance.md) | Performance | Use && instead of & for scalar logical AND | Info | No |
| [`OR2`](rules/performance.md) | Performance | Use \|\| instead of \| for scalar logical OR | Info | No |
| [`CLALL`](rules/performance.md) | Performance | Avoid clear all; it usually decreases performance | Info | No |
| [`CLCLS`](rules/performance.md) | Performance | Avoid clear classes | Info | No |
| [`CLFUNC`](rules/performance.md) | Performance | Avoid clear functions | Info | No |
| [`CLJAVA`](rules/performance.md) | Performance | Avoid clear java | Info | No |
| [`CLMEX`](rules/performance.md) | Performance | Avoid clear mex | Info | No |
| [`CLEAR0ARGS`](rules/performance.md) | Performance | clear with no arguments is often unnecessary | Info | No |
| [`ASGSL`](rules/readability.md) | Readability | Assignment inside a conditional expression | Info | No |
| [`COMNL`](rules/readability.md) | Readability | Newline following comma acts as a row separator in a matrix | Info | No |
| [`SPERR`](rules/readability.md) | Readability | Prefer a message identifier for error | Info | No |
| [`SPWRN`](rules/readability.md) | Readability | Prefer a message identifier for warning | Info | No |
| [`NCHKE`](rules/readability.md) | Readability | Use narginchk/nargoutchk for argument validation | Info | No |
| [`DSPSP`](rules/readability.md) | Readability | Prefer fprintf/disp over sprintf+disp for display | Info | No |
| [`DSPSY`](rules/readability.md) | Readability | Prefer fprintf/disp over system-based display | Info | No |
| [`STLOW`](rules/readability.md) | Readability | Unnecessary UPPER/LOWER call in a comparison | Info | No |
| [`FLUDLR`](rules/readability.md) | Readability | Nested flipud(fliplr(x))/fliplr(flipud(x)) should use rot90(x, 2) | Info | No |
| [`RPMT1`](rules/readability.md) | Readability | Trivial multiplication by 1 | Info | No |
| [`RPMT0`](rules/readability.md) | Readability | Trivial addition/subtraction of 0 | Info | No |
| [`RPMTT`](rules/readability.md) | Readability | Boolean tautology (true \|\| ...) | Info | No |
| [`RPMTF`](rules/readability.md) | Readability | Boolean contradiction (false && ...) | Info | No |
| [`RPMTI`](rules/readability.md) | Readability | Trivial multiplication by an identity-like expression | Info | No |
| [`RPMTN`](rules/readability.md) | Readability | Trivial negation patterns | Info | No |
| [`PSIZE`](rules/readability.md) | Readability | Use numel instead of prod(size(x)) | Info | No |
| [`LOGSUM`](rules/readability.md) | Readability | Use nnz instead of sum for logical vectors | Info | No |
| [`LOGL`](rules/readability.md) | Readability | Prefer any/all over manual logical reduction | Info | No |
| [`ISCHR`](rules/readability.md) | Readability | Use ischar(x) instead of isa(x,'char') | Info | No |
| [`ISSTR`](rules/readability.md) | Readability | Use isstring(x) instead of isa(x,'string') | Info | No |
| [`ISLOG`](rules/readability.md) | Readability | Use islogical(x) instead of isa(x,'logical') | Info | No |
| [`ISCEL`](rules/readability.md) | Readability | Use iscell(x) instead of isa(x,'cell') | Info | No |
| [`IJCL`](rules/readability.md) | Readability | i or j used as a variable (shadows the complex unit) | Info | No |
| [`ISMAT`](rules/readability.md) | Readability | Use ismatrix(x) instead of ndims(x)==2 | Info | No |
| [`ISROW`](rules/readability.md) | Readability | Use isrow(x) instead of size(x,1)==1 | Info | No |
| [`ISCOL`](rules/readability.md) | Readability | Use iscolumn(x) instead of size(x,2)==1 | Info | No |
| [`NBRAK2`](rules/readability.md) | Readability | Unnecessary brackets in indexing | Info | No |
| [`MFAMB`](rules/readability.md) | Readability | Cannot determine whether a name is a variable or function | Info | No |
| [`FVINR`](rules/readability.md) | Readability | Add an (Input) attribute to arguments blocks for readability | Info | No |
| [`STREMP`](rules/readability.md) | Readability | Use strlength(s)==0 instead of strcmp(s,'') | Info | No |
| [`STRCL1`](rules/readability.md) | Readability | Use strlength/strtrim instead of string-cleaning wrappers | Info | No |
| [`STRCLFH`](rules/readability.md) | Readability | Use strip instead of string-cleaning wrappers | Info | No |
| [`STRIFCND`](rules/readability.md) | Readability | Simplify if-conditions involving string comparisons | Info | No |
| [`CHARTEN`](rules/readability.md) | Readability | Use newline instead of char(10) | Info | No |
| [`SPRINTFN`](rules/readability.md) | Readability | Use num2str over simple sprintf for number formatting | Info | No |
| [`NOCOMMA`](rules/formatting.md) | Formatting | Use commas to separate elements in a row | Info | No |
| [`NO4LP`](rules/formatting.md) | Formatting | Use 4-space indentation in loop/conditional bodies | Info | No |
| [`ALIGN`](rules/formatting.md) | Formatting | Code alignment suggestion (elseif/else vs. if) | Info | No |
| [`NOPTS`](rules/formatting.md) | Formatting | Add parentheses around condition in if/while | Info | No |
| [`NOPRT`](rules/formatting.md) | Formatting | Remove unnecessary parentheses | Info | No |
| [`PRTCAL`](rules/formatting.md) | Formatting | Consider using command syntax instead of function syntax | Info | No |
| [`NCOMMA`](rules/formatting.md) | Formatting | Use comma to separate input arguments | Info | No |
| [`NODEF`](rules/unset-variables.md) | Unset Variables | NODEF | Warning | No |
| [`USENS`](rules/unset-variables.md) | Unset Variables | USENS | Warning | No |
| [`PSET`](rules/unset-variables.md) | Unset Variables | PSET | Warning | No |
| [`SUSENS`](rules/unset-variables.md) | Unset Variables | SUSENS | Warning | No |
| [`SVNODEF`](rules/unset-variables.md) | Unset Variables | SVNODEF | Warning | No |
| [`STOUT`](rules/unset-variables.md) | Unset Variables | STOUT | Warning | No |
| [`NASGU`](rules/unused.md) | Unused Constructions | Variable is assigned but never used | Warning | No |
| [`NUSED`](rules/unused.md) | Unused Constructions | Variable is defined (input arg) but never used | Warning | No |
| [`NOEFF`](rules/unused.md) | Unused Constructions | Statement has no effect (expression result discarded) | Warning | No |
| [`EQEFF`](rules/unused.md) | Unused Constructions | Comparison has no effect (result not used) | Warning | No |
| [`ASGLU`](rules/unused.md) | Unused Constructions | Assignment to a variable that is immediately overwritten | Warning | No |
| [`SETNU`](rules/unused.md) | Unused Constructions | Output of function assigned but never used | Warning | No |
| [`PUSE`](rules/unused.md) | Unused Constructions | Persistent/global variable set but not used | Warning | No |
| [`PREALL`](rules/unused.md) | Unused Constructions | Variable preallocated but unused | Warning | No |
| [`INUSA`](rules/unused.md) | Unused Constructions | Input argument not used in function | Warning | No |
| [`INUSD`](rules/unused.md) | Unused Constructions | Input argument defined but could be removed | Warning | No |
| [`VANUS`](rules/unused.md) | Unused Constructions | Value assigned to ans is unused | Warning | No |
| [`DEFNU`](rules/unused.md) | Unused Constructions | Local function defined but never called | Warning | No |
| [`UNRCH`](rules/unused.md) | Unused Constructions | Unreachable code after return/break/continue | Warning | No |
| [`MANU`](rules/unused.md) | Unused Constructions | Method defined but never called | Warning | No |
| [`VUNUS`](rules/unused.md) | Unused Constructions | Variable assigned in all branches but unused after | Warning | No |
| [`MSNU`](rules/unused.md) | Unused Constructions | Struct field set but never read | Warning | No |
| [`MSNE`](rules/unused.md) | Unused Constructions | Struct field doesn't exist (assigned but typo) | Warning | No |
| [`EMVDF`](rules/codegen.md) | Code Generation | EMVDF | Error | No |
| [`EMGRO`](rules/codegen.md) | Code Generation | EMGRO | Error | No |
| [`EMNODEF`](rules/codegen.md) | Code Generation | EMNODEF | Error | No |
| [`EMFCN`](rules/codegen.md) | Code Generation | EMFCN | Error | No |
| [`EMCEL`](rules/codegen.md) | Code Generation | EMCEL | Error | No |
| [`EMTC`](rules/codegen.md) | Code Generation | EMTC | Error | No |
| [`EMIMP`](rules/codegen.md) | Code Generation | EMIMP | Error | No |
| [`EMNST`](rules/codegen.md) | Code Generation | EMNST | Error | No |
| [`EMSCR`](rules/codegen.md) | Code Generation | EMSCR | Error | No |
| [`EMBRK`](rules/codegen.md) | Code Generation | EMBRK | Error | No |
| [`EMCNT`](rules/codegen.md) | Code Generation | EMCNT | Error | No |
| [`EMPFR`](rules/codegen.md) | Code Generation | EMPFR | Error | No |
| [`EMRTN`](rules/codegen.md) | Code Generation | EMRTN | Error | No |
| [`EMWHL`](rules/codegen.md) | Code Generation | EMWHL | Error | No |
| [`EMRIFAV`](rules/codegen.md) | Code Generation | EMRIFAV | Error | No |
| [`EMLOAD`](rules/codegen.md) | Code Generation | EMLOAD | Error | No |
| [`EMS2N`](rules/codegen.md) | Code Generation | EMS2N | Error | No |
| [`PRMNOIN`](rules/codegen.md) | Code Generation | PRMNOIN | Error | No |
| [`LOOPPRAGMAWITHOUTFOR`](rules/codegen.md) | Code Generation | LOOPPRAGMAWITHOUTFOR | Error | No |
| [`FPASE`](rules/codegen.md) | Code Generation | FPASE | Error | No |
| [`MCCD`](rules/deployment.md) | Deployment | MCCD | Warning | No |
| [`MCPRD`](rules/deployment.md) | Deployment | MCPRD | Warning | No |
| [`MCHLP`](rules/deployment.md) | Deployment | MCHLP | Warning | No |
| [`MCKBD`](rules/deployment.md) | Deployment | MCKBD | Warning | No |
| [`MCSVP`](rules/deployment.md) | Deployment | MCSVP | Warning | No |
| [`MCMLR`](rules/deployment.md) | Deployment | MCMLR | Warning | No |
| [`MCABF`](rules/deployment.md) | Deployment | MCABF | Warning | No |
| [`MCMFL`](rules/deployment.md) | Deployment | MCMFL | Warning | No |
| [`MCTBX`](rules/deployment.md) | Deployment | MCTBX | Warning | No |
| [`MCLL`](rules/deployment.md) | Deployment | MCLL | Warning | No |
| [`SONUMIN`](rules/system-objects.md) | System Objects | SONUMIN | Warning | No |
| [`SONUMOUT`](rules/system-objects.md) | System Objects | SONUMOUT | Warning | No |
| [`SODEPPROP`](rules/system-objects.md) | System Objects | SODEPPROP | Warning | No |
| [`SOINITPROP`](rules/system-objects.md) | System Objects | SOINITPROP | Warning | No |
| [`SODFLTVAL`](rules/system-objects.md) | System Objects | SODFLTVAL | Warning | No |
| [`SORSRVDNM`](rules/system-objects.md) | System Objects | SORSRVDNM | Warning | No |
| [`SOTUNPROP1`](rules/system-objects.md) | System Objects | SOTUNPROP1 | Warning | No |
| [`SOTUNPROP3`](rules/system-objects.md) | System Objects | SOTUNPROP3 | Warning | No |
| [`SOTUNPROP4`](rules/system-objects.md) | System Objects | SOTUNPROP4 | Warning | No |
| [`MCADE`](rules/unsupported.md) | Unsupported | MCADE | Warning | No |
| [`AWTIUD`](rules/unsupported.md) | Unsupported | AWTIUD | Warning | No |
| [`AXCHUD`](rules/unsupported.md) | Unsupported | AXCHUD | Warning | No |
| [`FEATUD`](rules/unsupported.md) | Unsupported | FEATUD | Warning | No |
| [`FNDPUD`](rules/unsupported.md) | Unsupported | FNDPUD | Warning | No |
| [`HGCNUD`](rules/unsupported.md) | Unsupported | HGCNUD | Warning | No |
| [`IMPKG`](rules/unsupported.md) | Unsupported | IMPKG | Warning | No |
| [`ISMBUD`](rules/unsupported.md) | Unsupported | ISMBUD | Warning | No |
| [`MIPKG`](rules/unsupported.md) | Unsupported | MIPKG | Warning | No |
| [`SEPTUD`](rules/unsupported.md) | Unsupported | SEPTUD | Warning | No |
| [`SYDEUD`](rules/unsupported.md) | Unsupported | SYDEUD | Warning | No |
| [`UIRSUD`](rules/unsupported.md) | Unsupported | UIRSUD | Warning | No |
| [`UISUUD`](rules/unsupported.md) | Unsupported | UISUUD | Warning | No |
| [`BDCFG`](rules/config-issues.md) | Configuration Issues | BDCFG | Error | No |
| [`CFERR`](rules/config-issues.md) | Configuration Issues | CFERR | Error | No |
| [`BDOPT`](rules/config-issues.md) | Configuration Issues | BDOPT | Error | No |
| [`CFIG`](rules/config-issues.md) | Configuration Issues | CFIG | Error | No |
| [`TMMSG`](rules/incomplete-analysis.md) | Incomplete Analysis | More than 10,000 diagnostics generated | Error | No |
| [`TMSMS`](rules/incomplete-analysis.md) | Incomplete Analysis | More than 1,000 parse errors generated | Error | No |
| [`MXASET`](rules/incomplete-analysis.md) | Incomplete Analysis | File too complex to analyze | Error | No |
| [`QUIT`](rules/incomplete-analysis.md) | Incomplete Analysis | Analysis did not complete | Error | No |
| [`NOSPC`](rules/incomplete-analysis.md) | Incomplete Analysis | File too complex (nesting) | Error | No |
| [`MBIG`](rules/incomplete-analysis.md) | Incomplete Analysis | File too large | Error | No |
| [`NOFIL`](rules/incomplete-analysis.md) | Incomplete Analysis | File not found | Error | No |
| [`MDOTM`](rules/incomplete-analysis.md) | Incomplete Analysis | Invalid file extension | Error | No |
| [`MDMCR`](rules/incomplete-analysis.md) | Incomplete Analysis | Deployed MATLAB file | Error | No |
| [`RDERR`](rules/incomplete-analysis.md) | Incomplete Analysis | Unable to read file | Error | No |
| [`EOFER`](rules/incomplete-analysis.md) | Incomplete Analysis | Too many syntax errors | Error | No |
| [`EOFMI`](rules/incomplete-analysis.md) | Incomplete Analysis | Incomplete file | Error | No |
| [`MDEEP`](rules/incomplete-analysis.md) | Incomplete Analysis | Parentheses/brackets nested too deeply | Error | No |
| [`DEEPC`](rules/incomplete-analysis.md) | Incomplete Analysis | Block comments nested too deeply | Error | No |
| [`DEEPN`](rules/incomplete-analysis.md) | Incomplete Analysis | Functions nested too deeply | Error | No |
| [`DEEPS`](rules/incomplete-analysis.md) | Incomplete Analysis | Statements nested too deeply | Error | No |
| [`TEXTL`](rules/incomplete-analysis.md) | Incomplete Analysis | Text too long | Error | No |
| [`SYNER`](rules/syntax-errors.md) | Syntax Errors | ERROR node detected in parse tree | Error | No |
| [`BDFIL`](rules/syntax-errors.md) | Syntax Errors | File name doesn't follow MATLAB naming rules | Error | No |
| [`BADNE`](rules/syntax-errors.md) | Syntax Errors | Source contains `!=` (MATLAB uses `~=`) | Error | No |
| [`BADOT`](rules/syntax-errors.md) | Syntax Errors | Source contains `..` not part of `...` | Error | No |
| [`TWOCM`](rules/syntax-errors.md) | Syntax Errors | Source contains `,,` | Error | No |
| [`CLIS`](rules/syntax-errors.md) | Syntax Errors | class_definition in a script file | Error | No |
| [`CLTWO`](rules/syntax-errors.md) | Syntax Errors | Multiple class_definition nodes in one file | Error | No |
| [`SOFOC`](rules/syntax-errors.md) | Syntax Errors | Statements outside a class_definition in a class file | Error | No |
| [`SEMFU`](rules/syntax-errors.md) | Syntax Errors | File has only empty statements (only `;` and whitespace) | Error | No |
| [`FNDOT`](rules/syntax-errors.md) | Syntax Errors | Function name contains dots outside class methods block | Error | No |
| [`FNSWA`](rules/syntax-errors.md) | Syntax Errors | Function name doesn't start with alphabetic character | Error | No |
| [`NOPAR2`](rules/syntax-errors.md) | Syntax Errors | Missing closing bracket mid-file | Error | No |
| [`EOLPAR`](rules/syntax-errors.md) | Syntax Errors | Missing closing bracket at end of line | Error | No |
| [`ENDPAR`](rules/syntax-errors.md) | Syntax Errors | Missing closing bracket at end of file | Error | No |
| [`ENDCT`](rules/syntax-errors.md) | Syntax Errors | ERROR node suggesting missing END | Error | No |
| [`ENDCT2`](rules/syntax-errors.md) | Syntax Errors | An END might be missing after a block-opening keyword | Error | No |
| [`ENDCT3`](rules/syntax-errors.md) | Syntax Errors | An END might be missing before a block-opening keyword | Error | No |
| [`ENDCT4`](rules/syntax-errors.md) | Syntax Errors | A METHODS block or END might be missing before a function definition | Error | No |
| [`NOLHS`](rules/syntax-errors.md) | Syntax Errors | Assignment with empty left side | Error | No |
| [`BADCH`](rules/syntax-errors.md) | Syntax Errors | Invalid control characters in source | Error | No |
| [`BADSP`](rules/syntax-errors.md) | Syntax Errors | Non-ASCII whitespace characters in source | Error | No |
| [`BADCT`](rules/syntax-errors.md) | Syntax Errors | Unicode explicit directional formatting characters | Error | No |
| [`REDEF`](rules/syntax-errors.md) | Syntax Errors | Same identifier used as both function name and variable | Error | No |
| [`SEPEXR`](rules/syntax-errors.md) | Syntax Errors | Missing newline/semicolon between statements | Error | No |
| [`SBTMP`](rules/syntax-errors.md) | Syntax Errors | Chaining outputs after parenthesis is not supported | Error | No |
| [`FVSYN`](rules/syntax-errors.md) | Syntax Errors | Invalid function argument syntax | Error | No |
| [`FVACI`](rules/syntax-errors.md) | Syntax Errors | Name-value arguments in cell indexing not supported | Error | No |
| [`FVACS`](rules/syntax-errors.md) | Syntax Errors | Quoted string used as name in name=value syntax | Error | No |
| [`FVAMI`](rules/syntax-errors.md) | Syntax Errors | Name in name=value syntax is not a valid identifier | Error | No |
| [`UNSET`](rules/syntax-errors.md) | Syntax Errors | Invalid use of operator on the left side of an assignment | Error | No |
| [`LHROW`](rules/syntax-errors.md) | Syntax Errors | Assignment left side cannot have multiple rows (';') | Error | No |
| [`RESWD`](rules/syntax-errors.md) | Syntax Errors | Invalid use of a reserved word | Error | No |
| [`SYNEND`](rules/syntax-errors.md) | Syntax Errors | Invalid use for END operator | Error | No |
| [`MCPLD`](rules/syntax-errors.md) | Syntax Errors | Invalid property syntax | Error | No |
| [`BADNOT`](rules/syntax-errors.md) | Syntax Errors | Using ~ to ignore a value is not permitted | Error | No |
| [`BADNOTLHS`](rules/syntax-errors.md) | Syntax Errors | Invalid use of logical not operator (~) on LHS | Error | No |
| [`STRIN`](rules/syntax-errors.md) | Syntax Errors | A quoted character vector is unterminated | Error | No |
| [`DOUQT`](rules/syntax-errors.md) | Syntax Errors | A double quoted string is unterminated | Error | No |
| [`INBLK`](rules/syntax-errors.md) | Syntax Errors | A block comment is unterminated at the end of the file | Error | No |
| [`BADFP`](rules/syntax-errors.md) | Syntax Errors | Invalid floating-point constant (e.g., truncated `1.2.3`) | Error | No |
| [`BADHBH`](rules/syntax-errors.md) | Syntax Errors | Invalid digit in a hexadecimal literal | Error | No |
| [`BADHBB`](rules/syntax-errors.md) | Syntax Errors | Invalid digit in a binary literal | Error | No |
| [`BADHBHT`](rules/syntax-errors.md) | Syntax Errors | Hex literal has too many digits for its type suffix | Error | No |
| [`BADHBBT`](rules/syntax-errors.md) | Syntax Errors | Binary literal has too many digits for its type suffix | Error | No |
| [`HEXTOOLONG`](rules/syntax-errors.md) | Syntax Errors | Hex literal has too many digits (max 16 without suffix) | Error | No |
| [`BINARYTOOLONG`](rules/syntax-errors.md) | Syntax Errors | Binary literal has too many digits (max 64 without suffix) | Error | No |
| [`VTPOD`](rules/syntax-errors.md) | Syntax Errors | Specify validation in the following order: size, then class, then functions | Error | No |
| [`TRYNC`](rules/good-practices.md#trync) | Good Practices | `try` without `catch` | Warning | No |
| [`CTCH`](rules/good-practices.md#ctch) | Good Practices | `catch` block is empty | Warning | No |
| [`WLAST`](rules/good-practices.md#wlast) | Good Practices | `warning` called as last statement in function | Warning | No |
| [`WNTAG`](rules/good-practices.md#wntag) | Good Practices | `warning` without message ID | Warning | No |
| [`ERTAG`](rules/good-practices.md#ertag) | Good Practices | `error` without message ID | Warning | No |
| [`STCMP`](rules/good-practices.md#stcmp) | Good Practices | Use `strcmp`/`strcmpi` instead of `==` for strings | Warning | No |
| [`STCI`](rules/good-practices.md#stci) | Good Practices | Use `strcmpi` for case-insensitive comparison | Warning | No |
| [`STISA`](rules/good-practices.md#stisa) | Good Practices | Use `isa` instead of `class` + `strcmp` | Warning | No |
| [`STRNU`](rules/good-practices.md#strnu) | Good Practices | Use `str2double` instead of `str2num` | Warning | No |
| [`EVLCS`](rules/good-practices.md#evlcs) | Good Practices | Avoid `eval` | Warning | No |
| [`EVLDOT`](rules/good-practices.md#evldot) | Good Practices | Avoid `eval` for dynamic field access | Warning | No |
| [`EVLEQ`](rules/good-practices.md#evleq) | Good Practices | Avoid `eval` for dynamic variable creation | Warning | No |
| [`EVLSYS`](rules/good-practices.md#evlsys) | Good Practices | Avoid `eval` for system commands | Warning | No |
| [`EVLDUAL`](rules/good-practices.md#evldual) | Good Practices | Avoid `evalin` | Warning | No |
| [`EVLSEQVAR`](rules/good-practices.md#evlseqvar) | Good Practices | Avoid `eval` to create sequential variables | Warning | No |
| [`NOANS`](rules/good-practices.md#noans) | Good Practices | Statement result assigned to `ans` | Warning | No |
| [`LOAD`](rules/good-practices.md#load) | Good Practices | `load` without output variable | Warning | No |
| [`SEPEX`](rules/good-practices.md#sepex) | Good Practices | Multiple statements on one line | Warning | No |
| [`NBRAK1`](rules/good-practices.md#nbrak1) | Good Practices | Unnecessary brackets around scalar | Warning | No |
| [`LNGNM`](rules/good-practices.md#lngnm) | Good Practices | Variable name exceeds length | Warning | No |
| [`CHAIN`](rules/good-practices.md#chain) | Good Practices | Method chaining on one line | Warning | No |
| [`DISPLAY`](rules/good-practices.md#display) | Good Practices | Override `display` is discouraged | Warning | No |
| [`FNDEF`](rules/good-practices.md#fndef) | Good Practices | Function not defined at expected location | Warning | No |
| [`NOIN`](rules/good-practices.md#noin) | Good Practices | Function has no input validation | Warning | No |
| [`VALST`](rules/good-practices.md#valst) | Good Practices | Validate function arguments | Warning | No |
| [`PROP`](rules/good-practices.md#prop) | Good Practices | Property validation missing | Warning | No |
| [`CPROP`](rules/good-practices.md#cprop) | Good Practices | Constant property could be method | Warning | No |
| [`FVAL`](rules/good-practices.md#fval) | Good Practices | Function value not used | Warning | No |
| [`FNCOLND`](rules/good-practices.md#fncolnd) | Good Practices | `end` used as column index without dimension | Warning | No |
| [`COMNC`](rules/good-practices.md#comnc) | Good Practices | Comment lacks space after `%` | Warning | No |
| [`ITERS`](rules/good-practices.md#iters) | Good Practices | Loop variable shadows outer variable | Warning | No |
| [`LOGPROD`](rules/good-practices.md#logprod) | Good Practices | Use `all` instead of `prod` on logical | Warning | No |
| [`LOGMIN`](rules/good-practices.md#logmin) | Good Practices | Use `all` instead of `min` on logical | Warning | No |
| [`LOGMAX`](rules/good-practices.md#logmax) | Good Practices | Use `any` instead of `max` on logical | Warning | No |
| [`ELARLOG`](rules/good-practices.md#elarlog) | Good Practices | Element-wise `&`/` | Warning | No |
| [`SHOCIRAA`](rules/good-practices.md#shociraa) | Good Practices | Short-circuit in array context | Warning | No |
| [`UNRPWR`](rules/good-practices.md#unrpwr) | Good Practices | Power of negative base may be complex | Warning | No |
| [`ADAPPREF`](rules/good-practices.md#adappref) | Good Practices | Avoid `addpref` (use settings) | Warning | No |
| [`KEYBOARDFUN`](rules/good-practices.md#keyboardfun) | Good Practices | `keyboard` left in code | Warning | No |
| [`GVMIS`](rules/good-practices.md#gvmis) | Good Practices | Global variable used but never declared | Warning | No |
| [`PFEVB`](rules/good-practices.md#pfevb) | Good Practices | Using EVALIN('base') or ASSIGNIN('base') inside a PARFOR loop refers to the worker machines' base workspaces | Warning | No |
| [`PFGP`](rules/good-practices.md#pfgp) | Good Practices | Avoid assigning to GLOBAL or PERSISTENT variable inside a PARFOR loop | Warning | No |
| [`PFGV`](rules/good-practices.md#pfgv) | Good Practices | Avoid using GLOBAL variable in a PARFOR loop | Warning | No |
| [`PFIIN`](rules/good-practices.md#pfiin) | Good Practices | The input variable should be initialized before the PARFOR loop | Warning | No |
| [`PFOUS`](rules/good-practices.md#pfous) | Good Practices | The output variable might not be used after the PARFOR loop | Warning | No |
| [`PFRNI`](rules/good-practices.md#pfrni) | Good Practices | Do not specify the increment explicitly; parfor can only use an increment of one | Warning | No |
| [`PFRIN`](rules/good-practices.md#pfrin) | Good Practices | The reduction variable might not be set before the PARFOR loop | Warning | No |
| [`PFRUS`](rules/good-practices.md#pfrus) | Good Practices | The reduction variable might not be used after the PARFOR loop | Warning | No |
| [`PFTUSW`](rules/good-practices.md#pftusw) | Good Practices | The temporary variable might be used after the PARFOR loop | Warning | No |
| [`PFUIXW`](rules/good-practices.md#pfuixw) | Good Practices | The index variable might be used after the PARFOR loop | Warning | No |
| [`SPEVB`](rules/good-practices.md#spevb) | Good Practices | Using EVALIN('base') or ASSIGNIN('base') inside an SPMD block refers to the worker machines' base workspaces | Warning | No |
| [`SPGV`](rules/good-practices.md#spgv) | Good Practices | Using the GLOBAL or PERSISTENT variable in an SPMD block might fail because it is accessed on a worker machine | Warning | No |
| [`DSPMDA`](rules/good-practices.md#dspmda) | Good Practices | Distributed array must be created outside of an SPMD block | Warning | No |
| [`COMFS`](rules/good-practices.md#comfs) | Good Practices | Comma makes the file a script, so functions are local | Warning | No |
| [`DUALC`](rules/good-practices.md#dualc) | Good Practices | Command might be prematurely ended by comma | Warning | No |
| [`RMFLD`](rules/good-practices.md#rmfld) | Good Practices | `rmfield` output must be assigned back to the structure | Warning | No |
| [`RMWRN`](rules/good-practices.md#rmwrn) | Good Practices | tag has been removed from MATLAB | Warning | No |
| [`SEMFS`](rules/good-practices.md#semfs) | Good Practices | Semicolon makes the file a script, so functions are local | Warning | No |
| [`STFLD`](rules/good-practices.md#stfld) | Good Practices | `setfield` output must be assigned back to the structure | Warning | No |
| [`STRSZ`](rules/good-practices.md#strsz) | Good Practices | Use `strcmp` to compare character vectors of different sizes | Warning | No |
| [`ATTF`](rules/good-practices.md#attf) | Good Practices | Unable to determine if the expression assigned to the `Abstract` attribute evaluates to true or false | Warning | No |
| [`ATTOF`](rules/good-practices.md#attof) | Good Practices | Setting the class attribute `Abstract` to false is not recommended | Warning | No |
| [`MCPO`](rules/good-practices.md#mcpo) | Good Practices | `SetObservable`/`GetObservable`/`AbortSet` property has no effect in a value class | Warning | No |
| [`MCSAC`](rules/good-practices.md#mcsac) | Good Practices | `SetAccess` cannot be set on Constant properties | Warning | No |
| [`MOBSRV`](rules/good-practices.md#mobsrv) | Good Practices | `SetObservable`/`GetObservable` on a Constant property has no effect | Warning | No |
| [`MDEPIN`](rules/good-practices.md#mdepin) | Good Practices | Default values should not be assigned to dependent properties | Warning | No |
| [`MCCPI`](rules/good-practices.md#mccpi) | Good Practices | Initialize the Constant property or make it an Abstract Constant property | Warning | No |
| [`MGMD`](rules/good-practices.md#mgmd) | Good Practices | `get` method should be implemented for each dependent property without private `GetAccess` | Warning | No |
| [`MCCPE`](rules/good-practices.md#mccpe) | Good Practices | Attempting to call a property or event as a function | Warning | No |
| [`MTHANS`](rules/good-practices.md#mthans) | Good Practices | Using `ANS` as a method name is not recommended | Warning | No |
| [`MHERM`](rules/good-practices.md#mherm) | Good Practices | Parenthesize the multiplication of a variable and its transpose | Warning | No |
| [`MNUML`](rules/good-practices.md#mnuml) | Good Practices | Use `VAR_NAME(numel(...), numel(...))` to create a square matrix | Warning | No |
| [`COMPNOP`](rules/good-practices.md#compnop) | Good Practices | Comparison with `true` simplifies to the function call itself | Warning | No |
| [`COMPNOT`](rules/good-practices.md#compnot) | Good Practices | Comparison with `~= true` or `== false` simplifies to `~call(...)` | Warning | No |
| [`M3COL`](rules/good-practices.md#m3col) | Good Practices | Three colons (`a:b:c:d`) in an expression is probably unintended | Warning | No |
| [`BDLGI`](rules/good-practices.md#bdlgi) | Good Practices | Variable might be set by a nonlogical operator | Warning | No |
| [`BDLOG1`](rules/good-practices.md#bdlog1) | Good Practices | Non-scalar logical value used in a conditional expression | Warning | No |
| [`BDLOG2`](rules/good-practices.md#bdlog2) | Good Practices | Scalar non-logical value used in a conditional expression | Warning | No |
| [`BDSCA`](rules/good-practices.md#bdsca) | Good Practices | `&&`/` | Warning | No |
| [`BDSCI`](rules/good-practices.md#bdsci) | Good Practices | Variable might be set by a nonscalar operator | Warning | No |
| [`MCHDP`](rules/good-practices.md#mchdp) | Good Practices | Property default that directly constructs a handle is shared by all instances | Warning | No |
| [`MCHDT`](rules/good-practices.md#mchdt) | Good Practices | Property default that resolves to a handle is shared by all instances | Warning | No |
| [`SHVAU`](rules/good-practices.md#shvau) | Good Practices | Ambiguous shared-variable usage between a nested function and its parent | Warning | No |
| [`GTARG`](rules/good-practices.md#gtarg) | Good Practices | Function might be called with too many arguments | Warning | No |
| [`LTARG`](rules/good-practices.md#ltarg) | Good Practices | Function might be called with too few arguments | Warning | No |
| [`CTPCT`](rules/good-practices.md#ctpct) | Good Practices | `sprintf`/`fprintf` format might not agree with the argument count | Warning | No |
| [`FXSET`](rules/good-practices.md#fxset) | Good Practices | Loop index variable is changed inside of a `for` loop | Warning | No |
| [`SIMPT`](rules/good-practices.md#simpt) | Good Practices | `import` statement does not run first in a function | Warning | No |
| [`TLEV`](rules/good-practices.md#tlev) | Good Practices | Dynamic-code function used as a sub-expression, not a top-level statement | Warning | No |
| [`UNONC`](rules/good-practices.md#unonc) | Good Practices | `onCleanup` output must be assigned to a variable, not `~` | Warning | No |
| [`MIPC1`](rules/good-practices.md#mipc1) | Good Practices | `computer('arch')` is platform-specific | Warning | No |
| [`SUBSINDEX`](rules/good-practices.md#subsindex) | Good Practices | Do not overload `subsindex` for fundamental data types | Warning | No |
| [`VTFIN`](rules/good-practices.md#vtfin) | Good Practices | Validated value should be the first input to a `validate*` function | Warning | No |
| [`CTOINW`](rules/good-practices.md#ctoinw) | Good Practices | Constructed object passed to its own constructor | Warning | No |
| [`FXUP`](rules/good-practices.md#fxup) | Good Practices | Outer loop index set inside a nested function | Warning | No |
| [`ADMTHDINV`](rules/good-practices.md#admthdinv) | Good Practices | Class method called without `app` as the first argument | Warning | No |
| [`ADPROP`](rules/good-practices.md#adprop) | Good Practices | Property assigned through a bare identifier instead of `app.PROP` | Warning | No |
| [`ADPROPLC`](rules/good-practices.md#adproplc) | Good Practices | Property read through a bare identifier instead of `app.PROP` | Warning | No |
| [`MCNPN`](rules/good-practices.md#mcnpn) | Good Practices | Member access on the object that is not declared in the class | Warning | No |
| [`MCNPR`](rules/good-practices.md#mcnpr) | Good Practices | Assignment target on the object that is not a property | Warning | No |
| [`MCSNOV`](rules/good-practices.md#mcsnov) | Good Practices | Value-class setter does not return the modified object | Warning | No |
| [`MCSOH`](rules/good-practices.md#mcsoh) | Good Practices | Handle-class setter unnecessarily returns the modified object | Warning | No |
| [`MCVM`](rules/good-practices.md#mcvm) | Good Practices | Value-class method modifying the object has no output | Warning | No |
| [`MCCSPS`](rules/good-practices.md#mccsps) | Good Practices | Constant property name used as a struct in a dot-access chain | Warning | No |
| [`MCSUP`](rules/good-practices.md#mcsup) | Good Practices | Setter accesses a property other than the one it sets | Warning | No |
| [`PFPF`](rules/language-spec.md#pfpf) | Language Specification | Nested parfor inside parfor | Error | No |
| [`PFSPMD`](rules/language-spec.md#pfspmd) | Language Specification | SPMD inside parfor | Error | No |
| [`PFBRK`](rules/language-spec.md#pfbrk) | Language Specification | break inside parfor | Error | No |
| [`PFRTN`](rules/language-spec.md#pfrtn) | Language Specification | return inside parfor | Error | No |
| [`PFGLOB`](rules/language-spec.md#pfglob) | Language Specification | global declaration inside parfor | Error | No |
| [`PFPERS`](rules/language-spec.md#pfpers) | Language Specification | persistent declaration inside parfor | Error | No |
| [`PFFORA`](rules/language-spec.md#pffora) | Language Specification | Assignment to for-loop variable inside parfor | Error | No |
| [`PFXST`](rules/language-spec.md#pfxst) | Language Specification | Assignment to parfor loop variable | Error | No |
| [`PFNF`](rules/language-spec.md#pfnf) | Language Specification | Nested function call inside parfor | Error | No |
| [`FPFORP`](rules/language-spec.md#fpforp) | Language Specification | fprintf writing to a file opened read-only | Error | No |
| [`FWFORP`](rules/language-spec.md#fwforp) | Language Specification | fwrite writing to a file opened read-only | Error | No |
| [`PFANON`](rules/language-spec.md#pfanon) | Language Specification | Sliced output variable used in an anonymous function | Error | No |
| [`PFANSLP`](rules/language-spec.md#pfanslp) | Language Specification | 'ans' as parfor loop variable | Error | No |
| [`PFANSNS`](rules/language-spec.md#pfansns) | Language Specification | 'ans' as for loop variable inside parfor | Error | No |
| [`PFCEL`](rules/language-spec.md#pfcel) | Language Specification | Function does not support cell arrays | Error | No |
| [`PFCTXT`](rules/language-spec.md#pfctxt) | Language Specification | Sliced variable indexed outside its defining for loop | Error | No |
| [`PFEVC`](rules/language-spec.md#pfevc) | Language Specification | EVALIN/ASSIGNIN('caller') invalid inside parfor | Error | No |
| [`PFFRNG`](rules/language-spec.md#pffrng) | Language Specification | Nested for range must be positive constants | Error | No |
| [`PFFSUB`](rules/language-spec.md#pffsub) | Language Specification | Indexing a nested for loop variable | Error | No |
| [`PFINCR`](rules/language-spec.md#pfincr) | Language Specification | Different reduction functions on the same variable | Error | No |
| [`PFINPT`](rules/language-spec.md#pfinpt) | Language Specification | 'inputname' not supported in parfor | Error | No |
| [`PFLD`](rules/language-spec.md#pfld) | Language Specification | 'load' must assign to an output in parfor | Error | No |
| [`PFMLTI`](rules/language-spec.md#pfmlti) | Language Specification | Nested for loop variable assigned in the body | Error | No |
| [`PFNACK`](rules/language-spec.md#pfnack) | Language Specification | narginchk/nargoutchk cannot be used in parfor | Error | No |
| [`PFNAIO`](rules/language-spec.md#pfnaio) | Language Specification | nargin/nargout require a function argument | Error | No |
| [`PFNAR`](rules/language-spec.md#pfnar) | Language Specification | Subtracting a reduction variable from expressions | Error | No |
| [`PFRFH`](rules/language-spec.md#pfrfh) | Language Specification | Reduction function must be a name or broadcast variable | Error | No |
| [`PFRNG`](rules/language-spec.md#pfrng) | Language Specification | Parfor range must be increasing consecutive integers | Error | No |
| [`PFSLO`](rules/language-spec.md#pfslo) | Language Specification | Variable indexed with parfor var is not a sliced output | Error | No |
| [`PFSLRD`](rules/language-spec.md#pfslrd) | Language Specification | Sliced indexing combined with non-indexed reads | Error | No |
| [`PFSLW`](rules/language-spec.md#pfslw) | Language Specification | Sliced accesses must all use the same subscripts | Error | No |
| [`PFSV`](rules/language-spec.md#pfsv) | Language Specification | SAVE requires '-fromstruct' in parfor | Error | No |
| [`PFUNK`](rules/language-spec.md#pfunk) | Language Specification | Parfor cannot run due to the way a variable is used | Error | No |
| [`PFUTMP`](rules/language-spec.md#pfutmp) | Language Specification | Temporary variable must be set before it is used | Error | No |
| [`PFUTVR`](rules/language-spec.md#pfutvr) | Language Specification | Variable intended as reduction but uninitialized | Error | No |
| [`PFVARS`](rules/language-spec.md#pfvars) | Language Specification | Parfor loop contains too many variables | Error | No |
| [`PFVSUB`](rules/language-spec.md#pfvsub) | Language Specification | Indexing the parfor loop variable | Error | No |
| [`PFANSRE`](rules/language-spec.md#pfansre) | Language Specification | 'ans' is not supported as a reduction variable | Error | No |
| [`PFANSSL`](rules/language-spec.md#pfanssl) | Language Specification | 'ans' is not supported as a sliced variable | Error | No |
| [`PFDF`](rules/language-spec.md#pfdf) | Language Specification | FOR with DRANGE becomes a conventional FOR inside a PARFOR | Error | No |
| [`PFPIE`](rules/language-spec.md#pfpie) | Language Specification | Valid indices for a sliced variable are restricted | Error | No |
| [`PFSAME`](rules/language-spec.md#pfsame) | Language Specification | Sliced variable indexed in different ways | Error | No |
| [`PFTIN`](rules/language-spec.md#pftin) | Language Specification | Temporary variable must be set before it is used | Error | No |
| [`ATUNK`](rules/language-spec.md#atunk) | Language Specification | Unknown attribute name | Error | No |
| [`ATLAB`](rules/language-spec.md#atlab) | Language Specification | 'Input'/'Output' attribute must not be valued or negated | Error | No |
| [`ATNAS`](rules/language-spec.md#atnas) | Language Specification | Meta-class attribute must be a meta-class or cell array | Error | No |
| [`ATNPI`](rules/language-spec.md#atnpi) | Language Specification | Class access attribute has an unexpected value | Error | No |
| [`ATNPP`](rules/language-spec.md#atnpp) | Language Specification | Events access attribute has an unexpected value | Error | No |
| [`ATPPI`](rules/language-spec.md#atppi) | Language Specification | Property access attribute has an unexpected value | Error | No |
| [`ATPPP`](rules/language-spec.md#atppp) | Language Specification | Method access attribute has an unexpected value | Error | No |
| [`ATAS`](rules/language-spec.md#atas) | Language Specification | Meta-class attribute value is unexpected | Error | No |
| [`ATVIZE`](rules/language-spec.md#atvize) | Language Specification | 'Visible' attribute is invalid for classes/events | Error | No |
| [`CLSAT`](rules/language-spec.md#clsat) | Language Specification | Specify class attributes before the class name | Error | No |
| [`CLSUNK`](rules/language-spec.md#clsunk) | Language Specification | Class or superclass could not be found on the path | Error | No |
| [`NOPRV`](rules/language-spec.md#noprv) | Language Specification | Class definition cannot be inside a private directory | Error | No |
| [`VTPCON`](rules/language-spec.md#vtpcon) | Language Specification | Validation functions must only use the property or literals | Error | No |
| [`VTPEAL`](rules/language-spec.md#vtpeal) | Language Specification | Specify at least one input argument for validator | Error | No |
| [`VTPIN`](rules/language-spec.md#vtpin) | Language Specification | Validation function must use the property as an input | Error | No |
| [`SPNST`](rules/language-spec.md#spnst) | Language Specification | parfor or spmd inside spmd | Error | No |
| [`SPRET`](rules/language-spec.md#spret) | Language Specification | return/break/continue inside spmd | Error | No |
| [`SPGP`](rules/language-spec.md#spgp) | Language Specification | global/persistent inside spmd | Error | No |
| [`MCFIL`](rules/language-spec.md#mcfil) | Language Specification | Class name and file name don't match | Error | No |
| [`MCDIR`](rules/language-spec.md#mcdir) | Language Specification | Class name and @directory name don't match | Error | No |
| [`MCRED`](rules/language-spec.md#mcred) | Language Specification | Property/event/enum name same as class name | Error | No |
| [`MCCBD`](rules/language-spec.md#mccbd) | Language Specification | Constructor not in class definition file | Error | No |
| [`MCS2I`](rules/language-spec.md#mcs2i) | Language Specification | Setter must have exactly 2 inputs | Error | No |
| [`MCS1O`](rules/language-spec.md#mcs1o) | Language Specification | Setter must have at most 1 output | Error | No |
| [`MCG1I`](rules/language-spec.md#mcg1i) | Language Specification | Getter must have exactly 1 input | Error | No |
| [`MCG1O`](rules/language-spec.md#mcg1o) | Language Specification | Getter must have exactly 1 output | Error | No |
| [`MCEB`](rules/language-spec.md#mceb) | Language Specification | Events defined in non-handle class | Error | No |
| [`MCANI`](rules/language-spec.md#mcani) | Language Specification | Abstract property initialized | Error | No |
| [`MCASC`](rules/language-spec.md#mcasc) | Language Specification | Abstract property in Sealed class | Error | No |
| [`MCSGA`](rules/language-spec.md#mcsga) | Language Specification | Set/get method in methods block with attributes | Error | No |
| [`MCSGP`](rules/language-spec.md#mcsgp) | Language Specification | Set/get method refers to invalid property | Error | No |
| [`MABSEAC`](rules/language-spec.md#mabseac) | Language Specification | Instance properties/methods illegal in Sealed+Abstract classes | Error | No |
| [`MABSEAM`](rules/language-spec.md#mabseam) | Language Specification | A method cannot be both Abstract and Sealed | Error | No |
| [`MCAPP`](rules/language-spec.md#mcapp) | Language Specification | Private property cannot be Abstract | Error | No |
| [`MCCBS`](rules/language-spec.md#mccbs) | Language Specification | Superclass constructor is not a declared superclass name | Error | No |
| [`MCCBU`](rules/language-spec.md#mccbu) | Language Specification | Superclass constructor called after object use | Error | No |
| [`MCCMC`](rules/language-spec.md#mccmc) | Language Specification | Constructor for superclass can only be called once | Error | No |
| [`MCCSOP`](rules/language-spec.md#mccsop) | Language Specification | Unable to modify Constant property | Error | No |
| [`MCGSA`](rules/language-spec.md#mcgsa) | Language Specification | Set/get method tries to access an abstract property | Error | No |
| [`MCMIO`](rules/language-spec.md#mcmio) | Language Specification | Method has too many inputs or outputs | Error | No |
| [`MCMSP`](rules/language-spec.md#mcmsp) | Language Specification | Private method cannot be Abstract | Error | No |
| [`MCMTP`](rules/language-spec.md#mcmtp) | Language Specification | TestParameterDefinition methods must be Static | Error | No |
| [`MCPIN`](rules/language-spec.md#mcpin) | Language Specification | Property initialized to instance of the class itself | Error | No |
| [`MCPSG`](rules/language-spec.md#mcpsg) | Language Specification | Set or get method must be fully defined in the class file | Error | No |
| [`MCSCC`](rules/language-spec.md#mcscc) | Language Specification | Superclass constructor call needs a matching subclass constructor name | Error | No |
| [`MCSCF`](rules/language-spec.md#mcscf) | Language Specification | Superclass constructor must be assigned to the first output | Error | No |
| [`MCSCM`](rules/language-spec.md#mcscm) | Language Specification | Superclass method call needs a matching method name | Error | No |
| [`MCSCN`](rules/language-spec.md#mcscn) | Language Specification | Method tries to set a constant property | Error | No |
| [`MCSCO`](rules/language-spec.md#mcsco) | Language Specification | Superclass constructor must use the first output argument | Error | No |
| [`MCSCT`](rules/language-spec.md#mcsct) | Language Specification | Superclass constructor call must not be conditionalized | Error | No |
| [`MCSMO`](rules/language-spec.md#mcsmo) | Language Specification | Multiple outputs from superclass initialization unsupported | Error | No |
| [`MCSWA`](rules/language-spec.md#mcswa) | Language Specification | Sealed class cannot specify allowed subclasses | Error | No |
| [`MTAGS3`](rules/language-spec.md#mtags3) | Language Specification | Access attribute conflicts with SetAccess/GetAccess | Error | No |
| [`MTMAT`](rules/language-spec.md#mtmat) | Language Specification | Attribute can only be set once | Error | No |
| [`MWKCL`](rules/language-spec.md#mwkcl) | Language Specification | WeakHandle property must have a class validation | Error | No |
| [`MWKCT`](rules/language-spec.md#mwkct) | Language Specification | WeakHandle and Constant attributes conflict | Error | No |
| [`MWKREF`](rules/language-spec.md#mwkref) | Language Specification | WeakHandle and Dependent attributes conflict | Error | No |
| [`FVNST`](rules/language-spec.md#fvnst) | Language Specification | Arguments blocks in nested functions | Error | No |
| [`FVAPN`](rules/language-spec.md#fvapn) | Language Specification | Move name=value name-value arguments to the end | Error | No |
| [`FVATF`](rules/language-spec.md#fvatf) | Language Specification | Attribute values in arguments blocks must be logical constants | Error | No |
| [`FVBTN`](rules/language-spec.md#fvbtn) | Language Specification | Use of this function is not supported in arguments blocks | Error | No |
| [`FVDAN`](rules/language-spec.md#fvdan) | Language Specification | Same name as name-value structure and positional | Error | No |
| [`FVDAP`](rules/language-spec.md#fvdap) | Language Specification | Positional argument can only be declared once | Error | No |
| [`FVDNF`](rules/language-spec.md#fvdnf) | Language Specification | Name-value argument can only be declared once | Error | No |
| [`FVDREP`](rules/language-spec.md#fvdrep) | Language Specification | Multiple Repeating arguments blocks not supported | Error | No |
| [`FVIDV`](rules/language-spec.md#fvidv) | Language Specification | Validation/default on ignored arguments unsupported | Error | No |
| [`FVIOA`](rules/language-spec.md#fvioa) | Language Specification | Both 'Input' and 'Output' attributes on one block | Error | No |
| [`FVMCL`](rules/language-spec.md#fvmcl) | Language Specification | Multiple name-value structures using .? syntax | Error | No |
| [`FVNDE`](rules/language-spec.md#fvnde) | Language Specification | Default values illegal for class-name name-value | Error | No |
| [`FVNIV`](rules/language-spec.md#fvniv) | Language Specification | Variable is not an input to the function | Error | No |
| [`FVNREP`](rules/language-spec.md#fvnrep) | Language Specification | Name-value arguments in Repeating block unsupported | Error | No |
| [`FVNSC`](rules/language-spec.md#fvnsc) | Language Specification | Calling nested functions in arguments blocks | Error | No |
| [`FVNVL`](rules/language-spec.md#fvnvl) | Language Specification | Validation illegal for class-name name-value | Error | No |
| [`FVOBI`](rules/language-spec.md#fvobi) | Language Specification | Declare input blocks before output blocks | Error | No |
| [`FVOCON`](rules/language-spec.md#fvocon) | Language Specification | Output validation only uses arg or literals | Error | No |
| [`FVOND`](rules/language-spec.md#fvond) | Language Specification | Name-value arguments in default values unsupported | Error | No |
| [`FVONV`](rules/language-spec.md#fvonv) | Language Specification | Name-value arguments without dotted name in validation | Error | No |
| [`FVOOD`](rules/language-spec.md#fvood) | Language Specification | Default value for output argument unsupported | Error | No |
| [`FVOOI`](rules/language-spec.md#fvooi) | Language Specification | Ignored arguments in output block unsupported | Error | No |
| [`FVOON`](rules/language-spec.md#fvoon) | Language Specification | Name-value argument as output unsupported | Error | No |
| [`FVORDI`](rules/language-spec.md#fvordi) | Language Specification | Ignored inputs after Repeating or name-value | Error | No |
| [`FVORDN`](rules/language-spec.md#fvordn) | Language Specification | Positional arguments before name-value arguments | Error | No |
| [`FVORDO`](rules/language-spec.md#fvordo) | Language Specification | Repeating outputs after required outputs | Error | No |
| [`FVORDP`](rules/language-spec.md#fvordp) | Language Specification | Positional order: required, optional, repeating | Error | No |
| [`FVORM`](rules/language-spec.md#fvorm) | Language Specification | Multiple repeating output arguments unsupported | Error | No |
| [`FVOVREP`](rules/language-spec.md#fvovrep) | Language Specification | varargout only in Repeating output block | Error | No |
| [`FVREPD`](rules/language-spec.md#fvrepd) | Language Specification | Defaults in Repeating block unsupported | Error | No |
| [`FVREPO`](rules/language-spec.md#fvrepo) | Language Specification | Repeating input block with varargin has no other args | Error | No |
| [`FVSOR`](rules/language-spec.md#fvsor) | Language Specification | Input block matches function line order | Error | No |
| [`FVSORO`](rules/language-spec.md#fvsoro) | Language Specification | Output block matches function line order | Error | No |
| [`FVUBD`](rules/language-spec.md#fvubd) | Language Specification | Argument referenced before declared | Error | No |
| [`FVVCON`](rules/language-spec.md#fvvcon) | Language Specification | Input validation only uses prior positionals | Error | No |
| [`FVVIN`](rules/language-spec.md#fvvin) | Language Specification | Validation function must use the argument | Error | No |
| [`FVVREP`](rules/language-spec.md#fvvrep) | Language Specification | varargin only inside repeating input block | Error | No |
| [`TINVALDIM`](rules/language-spec.md#tinvaldim) | Language Specification | Each dimension must be nonnegative integer or colon | Error | No |
| [`TTOOFEWDIMS`](rules/language-spec.md#ttoofewdims) | Language Specification | Specify at least two dimensions for size | Error | No |
| [`FCONV`](rules/language-spec.md#fconv) | Language Specification | Variable name same as script name | Error | No |
| [`FCONF`](rules/language-spec.md#fconf) | Language Specification | Local function name same as file name | Error | No |
| [`GPFST`](rules/language-spec.md#gpfst) | Language Specification | Global/persistent must precede first use | Error | No |
| [`GPNES`](rules/language-spec.md#gpnes) | Language Specification | Global/persistent must be in outermost function | Error | No |
| [`NPERS`](rules/language-spec.md#npers) | Language Specification | Persistent in script | Error | No |
| [`ROWLN`](rules/language-spec.md#rowln) | Language Specification | Matrix rows must be same length | Error | No |
| [`FCNANS`](rules/language-spec.md#fcnans) | Language Specification | Function named 'ans' | Error | No |
| [`CLANS`](rules/language-spec.md#clans) | Language Specification | Class named 'ans' | Error | No |
| [`BRKFOR`](rules/language-spec.md#brkfor) | Language Specification | break outside loop | Error | No |
| [`CONTFOR`](rules/language-spec.md#contfor) | Language Specification | continue outside loop | Error | No |
| [`IDXCOLND`](rules/language-spec.md#idxcolnd) | Language Specification | END operator outside index expression | Error | No |
| [`CTOINE`](rules/language-spec.md#ctoine) | Language Specification | Use of constructed object as input to constructor is not supported | Error | No |
| [`CTORO`](rules/language-spec.md#ctoro) | Language Specification | Class constructors must be declared with at least one output argument | Error | No |
| [`ERTXT`](rules/language-spec.md#ertxt) | Language Specification | Specify an error message with the message identifier | Error | No |
| [`MHERIT`](rules/language-spec.md#mherit) | Language Specification | Deriving from a built-in MATLAB class is not supported | Error | No |
| [`NCHKOS`](rules/language-spec.md#nchkos) | Language Specification | NARGINCHK does not return any values | Error | No |
| [`SPBFN`](rules/language-spec.md#spbfn) | Language Specification | Non-transparent workspace access inside spmd | Error | No |
| [`SPBRK`](rules/language-spec.md#spbrk) | Language Specification | break/continue not fully contained in spmd (subsumed by SPRET) | Error | No |
| [`SPDEC`](rules/language-spec.md#spdec) | Language Specification | SPMD worker bounds must be nonnegative integers | Error | No |
| [`SPDEC3`](rules/language-spec.md#spdec3) | Language Specification | SPMD can only specify lower and upper worker bounds | Error | No |
| [`SPEVC`](rules/language-spec.md#spevc) | Language Specification | EVALIN/ASSIGNIN('caller') invalid inside spmd | Error | No |
| [`SPLD`](rules/language-spec.md#spld) | Language Specification | Assign the output of LOAD in spmd blocks | Error | No |
| [`SPNF`](rules/language-spec.md#spnf) | Language Specification | Nested function call inside spmd | Error | No |
| [`SPSV`](rules/language-spec.md#spsv) | Language Specification | SAVE requires '-fromstruct' inside spmd | Error | No |
| [`SPWHOS`](rules/language-spec.md#spwhos) | Language Specification | who/whos without '-file' invalid inside spmd | Error | No |
| [`USESWNS`](rules/language-spec.md#useswns) | Language Specification | Variable must be explicitly defined before first use | Error | No |
| [`WTXT`](rules/language-spec.md#wtxt) | Language Specification | Specify a warning message with the message identifier | Error | No |
| [`SYSBANG`](rules/custom-checks.md#sysbang) | Custom Checks | System command used (!) | Warning | No |
| [`FCNIL`](rules/custom-checks.md#fcnil) | Custom Checks | Function input count exceeds limit | Warning | No |
| [`FCNOL`](rules/custom-checks.md#fcnol) | Custom Checks | Function output count exceeds limit | Warning | No |
| [`FCNLL`](rules/custom-checks.md#fcnll) | Custom Checks | Function line count exceeds limit | Warning | No |
| [`LLMNC`](rules/custom-checks.md#llmnc) | Custom Checks | Line length exceeds limit | Warning | No |
| [`MNCSN`](rules/custom-checks.md#mncsn) | Custom Checks | Statement nesting depth exceeds limit | Warning | No |
| [`DAFTC`](rules/custom-checks.md#daftc) | Custom Checks | Too many children in tree | Warning | No |
| [`DAFPV`](rules/custom-checks.md#dafpv) | Custom Checks | Too many persistent variables | Warning | No |
| [`DAFCO`](rules/custom-checks.md#dafco) | Custom Checks | Too many conditions in expression | Warning | No |
| [`DAFBR`](rules/custom-checks.md#dafbr) | Custom Checks | Too many branches in switch/if | Warning | No |
| [`DAFRT`](rules/custom-checks.md#dafrt) | Custom Checks | Too many return points | Warning | No |
| [`DAFSC`](rules/custom-checks.md#dafsc) | Custom Checks | Too many semicolons on one line | Warning | No |
| [`DAFNF`](rules/custom-checks.md#dafnf) | Custom Checks | Too many nested functions | Warning | No |
| [`DAFCF`](rules/custom-checks.md#dafcf) | Custom Checks | Too many called functions | Warning | No |
| [`DAFAF`](rules/custom-checks.md#dafaf) | Custom Checks | Too many anonymous functions | Warning | No |
| [`DAFCV`](rules/custom-checks.md#dafcv) | Custom Checks | Too many local variables | Warning | No |
| [`DAFCVC`](rules/custom-checks.md#dafcvc) | Custom Checks | Too many local constants | Warning | No |
| [`DAFVI`](rules/custom-checks.md#dafvi) | Custom Checks | Too many input arguments used | Warning | No |
| [`DAFVO`](rules/custom-checks.md#dafvo) | Custom Checks | Too many output arguments used | Warning | No |
| [`CYCCOM`](rules/custom-checks.md#cyccom) | Custom Checks | Cyclomatic complexity of function exceeds limit | Warning | No |
| [`SCYCCOM`](rules/custom-checks.md#scyccom) | Custom Checks | Strict cyclomatic complexity exceeds limit | Warning | No |
| [`ACYCCOM`](rules/custom-checks.md#acyccom) | Custom Checks | Average cyclomatic complexity exceeds limit | Warning | No |
| [`MCYCCOM`](rules/custom-checks.md#mcyccom) | Custom Checks | Method cyclomatic complexity exceeds limit | Warning | No |
| [`MSCYCCOM`](rules/custom-checks.md#mscyccom) | Custom Checks | Method strict cyclomatic complexity exceeds limit | Warning | No |
| [`MACYCCOM`](rules/custom-checks.md#macyccom) | Custom Checks | Method average cyclomatic complexity exceeds limit | Warning | No |
| [`naming.class.maxLength`](rules/naming.md#namingclassmaxlength) | Naming | Naming check for class maxLength | Info | No |
| [`naming.class.minLength`](rules/naming.md#namingclassminlength) | Naming | Naming check for class minLength | Info | No |
| [`naming.class.regularExpression`](rules/naming.md#namingclassregularexpression) | Naming | Naming check for class regularExpression | Info | No |
| [`naming.class.requiredPrefix`](rules/naming.md#namingclassrequiredprefix) | Naming | Naming check for class requiredPrefix | Info | No |
| [`naming.class.disallowedPrefix`](rules/naming.md#namingclassdisallowedprefix) | Naming | Naming check for class disallowedPrefix | Info | No |
| [`naming.class.disallowedPhrase`](rules/naming.md#namingclassdisallowedphrase) | Naming | Naming check for class disallowedPhrase | Info | No |
| [`naming.class.requiredSuffix`](rules/naming.md#namingclassrequiredsuffix) | Naming | Naming check for class requiredSuffix | Info | No |
| [`naming.class.disallowedSuffix`](rules/naming.md#namingclassdisallowedsuffix) | Naming | Naming check for class disallowedSuffix | Info | No |
| [`naming.class.casing`](rules/naming.md#namingclasscasing) | Naming | Naming check for class casing | Info | No |
| [`naming.function.maxLength`](rules/naming.md#namingfunctionmaxlength) | Naming | Naming check for function maxLength | Info | No |
| [`naming.function.minLength`](rules/naming.md#namingfunctionminlength) | Naming | Naming check for function minLength | Info | No |
| [`naming.function.regularExpression`](rules/naming.md#namingfunctionregularexpression) | Naming | Naming check for function regularExpression | Info | No |
| [`naming.function.requiredPrefix`](rules/naming.md#namingfunctionrequiredprefix) | Naming | Naming check for function requiredPrefix | Info | No |
| [`naming.function.disallowedPrefix`](rules/naming.md#namingfunctiondisallowedprefix) | Naming | Naming check for function disallowedPrefix | Info | No |
| [`naming.function.disallowedPhrase`](rules/naming.md#namingfunctiondisallowedphrase) | Naming | Naming check for function disallowedPhrase | Info | No |
| [`naming.function.requiredSuffix`](rules/naming.md#namingfunctionrequiredsuffix) | Naming | Naming check for function requiredSuffix | Info | No |
| [`naming.function.disallowedSuffix`](rules/naming.md#namingfunctiondisallowedsuffix) | Naming | Naming check for function disallowedSuffix | Info | No |
| [`naming.function.casing`](rules/naming.md#namingfunctioncasing) | Naming | Naming check for function casing | Info | No |
| [`naming.localFunction.maxLength`](rules/naming.md#naminglocalfunctionmaxlength) | Naming | Naming check for localFunction maxLength | Info | No |
| [`naming.localFunction.minLength`](rules/naming.md#naminglocalfunctionminlength) | Naming | Naming check for localFunction minLength | Info | No |
| [`naming.localFunction.regularExpression`](rules/naming.md#naminglocalfunctionregularexpression) | Naming | Naming check for localFunction regularExpression | Info | No |
| [`naming.localFunction.requiredPrefix`](rules/naming.md#naminglocalfunctionrequiredprefix) | Naming | Naming check for localFunction requiredPrefix | Info | No |
| [`naming.localFunction.disallowedPrefix`](rules/naming.md#naminglocalfunctiondisallowedprefix) | Naming | Naming check for localFunction disallowedPrefix | Info | No |
| [`naming.localFunction.disallowedPhrase`](rules/naming.md#naminglocalfunctiondisallowedphrase) | Naming | Naming check for localFunction disallowedPhrase | Info | No |
| [`naming.localFunction.requiredSuffix`](rules/naming.md#naminglocalfunctionrequiredsuffix) | Naming | Naming check for localFunction requiredSuffix | Info | No |
| [`naming.localFunction.disallowedSuffix`](rules/naming.md#naminglocalfunctiondisallowedsuffix) | Naming | Naming check for localFunction disallowedSuffix | Info | No |
| [`naming.localFunction.casing`](rules/naming.md#naminglocalfunctioncasing) | Naming | Naming check for localFunction casing | Info | No |
| [`naming.method.maxLength`](rules/naming.md#namingmethodmaxlength) | Naming | Naming check for method maxLength | Info | No |
| [`naming.method.minLength`](rules/naming.md#namingmethodminlength) | Naming | Naming check for method minLength | Info | No |
| [`naming.method.regularExpression`](rules/naming.md#namingmethodregularexpression) | Naming | Naming check for method regularExpression | Info | No |
| [`naming.method.requiredPrefix`](rules/naming.md#namingmethodrequiredprefix) | Naming | Naming check for method requiredPrefix | Info | No |
| [`naming.method.disallowedPrefix`](rules/naming.md#namingmethoddisallowedprefix) | Naming | Naming check for method disallowedPrefix | Info | No |
| [`naming.method.disallowedPhrase`](rules/naming.md#namingmethoddisallowedphrase) | Naming | Naming check for method disallowedPhrase | Info | No |
| [`naming.method.requiredSuffix`](rules/naming.md#namingmethodrequiredsuffix) | Naming | Naming check for method requiredSuffix | Info | No |
| [`naming.method.disallowedSuffix`](rules/naming.md#namingmethoddisallowedsuffix) | Naming | Naming check for method disallowedSuffix | Info | No |
| [`naming.method.casing`](rules/naming.md#namingmethodcasing) | Naming | Naming check for method casing | Info | No |
| [`naming.nestedFunction.maxLength`](rules/naming.md#namingnestedfunctionmaxlength) | Naming | Naming check for nestedFunction maxLength | Info | No |
| [`naming.nestedFunction.minLength`](rules/naming.md#namingnestedfunctionminlength) | Naming | Naming check for nestedFunction minLength | Info | No |
| [`naming.nestedFunction.regularExpression`](rules/naming.md#namingnestedfunctionregularexpression) | Naming | Naming check for nestedFunction regularExpression | Info | No |
| [`naming.nestedFunction.requiredPrefix`](rules/naming.md#namingnestedfunctionrequiredprefix) | Naming | Naming check for nestedFunction requiredPrefix | Info | No |
| [`naming.nestedFunction.disallowedPrefix`](rules/naming.md#namingnestedfunctiondisallowedprefix) | Naming | Naming check for nestedFunction disallowedPrefix | Info | No |
| [`naming.nestedFunction.disallowedPhrase`](rules/naming.md#namingnestedfunctiondisallowedphrase) | Naming | Naming check for nestedFunction disallowedPhrase | Info | No |
| [`naming.nestedFunction.requiredSuffix`](rules/naming.md#namingnestedfunctionrequiredsuffix) | Naming | Naming check for nestedFunction requiredSuffix | Info | No |
| [`naming.nestedFunction.disallowedSuffix`](rules/naming.md#namingnestedfunctiondisallowedsuffix) | Naming | Naming check for nestedFunction disallowedSuffix | Info | No |
| [`naming.nestedFunction.casing`](rules/naming.md#namingnestedfunctioncasing) | Naming | Naming check for nestedFunction casing | Info | No |
| [`naming.property.maxLength`](rules/naming.md#namingpropertymaxlength) | Naming | Naming check for property maxLength | Info | No |
| [`naming.property.minLength`](rules/naming.md#namingpropertyminlength) | Naming | Naming check for property minLength | Info | No |
| [`naming.property.regularExpression`](rules/naming.md#namingpropertyregularexpression) | Naming | Naming check for property regularExpression | Info | No |
| [`naming.property.requiredPrefix`](rules/naming.md#namingpropertyrequiredprefix) | Naming | Naming check for property requiredPrefix | Info | No |
| [`naming.property.disallowedPrefix`](rules/naming.md#namingpropertydisallowedprefix) | Naming | Naming check for property disallowedPrefix | Info | No |
| [`naming.property.disallowedPhrase`](rules/naming.md#namingpropertydisallowedphrase) | Naming | Naming check for property disallowedPhrase | Info | No |
| [`naming.property.requiredSuffix`](rules/naming.md#namingpropertyrequiredsuffix) | Naming | Naming check for property requiredSuffix | Info | No |
| [`naming.property.disallowedSuffix`](rules/naming.md#namingpropertydisallowedsuffix) | Naming | Naming check for property disallowedSuffix | Info | No |
| [`naming.property.casing`](rules/naming.md#namingpropertycasing) | Naming | Naming check for property casing | Info | No |
| [`naming.event.maxLength`](rules/naming.md#namingeventmaxlength) | Naming | Naming check for event maxLength | Info | No |
| [`naming.event.minLength`](rules/naming.md#namingeventminlength) | Naming | Naming check for event minLength | Info | No |
| [`naming.event.regularExpression`](rules/naming.md#namingeventregularexpression) | Naming | Naming check for event regularExpression | Info | No |
| [`naming.event.requiredPrefix`](rules/naming.md#namingeventrequiredprefix) | Naming | Naming check for event requiredPrefix | Info | No |
| [`naming.event.disallowedPrefix`](rules/naming.md#namingeventdisallowedprefix) | Naming | Naming check for event disallowedPrefix | Info | No |
| [`naming.event.disallowedPhrase`](rules/naming.md#namingeventdisallowedphrase) | Naming | Naming check for event disallowedPhrase | Info | No |
| [`naming.event.requiredSuffix`](rules/naming.md#namingeventrequiredsuffix) | Naming | Naming check for event requiredSuffix | Info | No |
| [`naming.event.disallowedSuffix`](rules/naming.md#namingeventdisallowedsuffix) | Naming | Naming check for event disallowedSuffix | Info | No |
| [`naming.event.casing`](rules/naming.md#namingeventcasing) | Naming | Naming check for event casing | Info | No |
| [`naming.enumeration.maxLength`](rules/naming.md#namingenumerationmaxlength) | Naming | Naming check for enumeration maxLength | Info | No |
| [`naming.enumeration.minLength`](rules/naming.md#namingenumerationminlength) | Naming | Naming check for enumeration minLength | Info | No |
| [`naming.enumeration.regularExpression`](rules/naming.md#namingenumerationregularexpression) | Naming | Naming check for enumeration regularExpression | Info | No |
| [`naming.enumeration.requiredPrefix`](rules/naming.md#namingenumerationrequiredprefix) | Naming | Naming check for enumeration requiredPrefix | Info | No |
| [`naming.enumeration.disallowedPrefix`](rules/naming.md#namingenumerationdisallowedprefix) | Naming | Naming check for enumeration disallowedPrefix | Info | No |
| [`naming.enumeration.disallowedPhrase`](rules/naming.md#namingenumerationdisallowedphrase) | Naming | Naming check for enumeration disallowedPhrase | Info | No |
| [`naming.enumeration.requiredSuffix`](rules/naming.md#namingenumerationrequiredsuffix) | Naming | Naming check for enumeration requiredSuffix | Info | No |
| [`naming.enumeration.disallowedSuffix`](rules/naming.md#namingenumerationdisallowedsuffix) | Naming | Naming check for enumeration disallowedSuffix | Info | No |
| [`naming.enumeration.casing`](rules/naming.md#namingenumerationcasing) | Naming | Naming check for enumeration casing | Info | No |
| [`naming.variable.maxLength`](rules/naming.md#namingvariablemaxlength) | Naming | Naming check for variable maxLength | Info | No |
| [`naming.variable.minLength`](rules/naming.md#namingvariableminlength) | Naming | Naming check for variable minLength | Info | No |
| [`naming.variable.regularExpression`](rules/naming.md#namingvariableregularexpression) | Naming | Naming check for variable regularExpression | Info | No |
| [`naming.variable.requiredPrefix`](rules/naming.md#namingvariablerequiredprefix) | Naming | Naming check for variable requiredPrefix | Info | No |
| [`naming.variable.disallowedPrefix`](rules/naming.md#namingvariabledisallowedprefix) | Naming | Naming check for variable disallowedPrefix | Info | No |
| [`naming.variable.disallowedPhrase`](rules/naming.md#namingvariabledisallowedphrase) | Naming | Naming check for variable disallowedPhrase | Info | No |
| [`naming.variable.requiredSuffix`](rules/naming.md#namingvariablerequiredsuffix) | Naming | Naming check for variable requiredSuffix | Info | No |
| [`naming.variable.disallowedSuffix`](rules/naming.md#namingvariabledisallowedsuffix) | Naming | Naming check for variable disallowedSuffix | Info | No |
| [`naming.variable.casing`](rules/naming.md#namingvariablecasing) | Naming | Naming check for variable casing | Info | No |

These rows are generated from the rule docstrings by `docs/scripts/gen_rules_docs.ts`. Rebuild the docs (`bun run docs:gen`) to refresh them.

## Data-Driven Check IDs

These tables are generated from the TOML data files by the TypeScript
generator in `docs/scripts/gen_rules_docs.ts`. Rebuild the docs (`bun run docs:gen`)
to refresh them whenever `data/compatibility.toml` or
`data/suggested_improvements.toml` changes.

### Compatibility Considerations (1012 checks)

| Check ID | Message |
| -------- | ------- |
| [`DPSD`](rules/compatibility.md#dpsd) | 'psd' has been removed. Use 'periodogram' or 'pwelch' instead. |
| [`DSPDF`](rules/compatibility.md#dspdf) | 'dsp.DigitalFilter' has been removed. Use 'dsp.FIRFilter', 'dsp.IIRFilter', or 'dsp.AllpoleFilter' instead. |
| [`DMSSPEC`](rules/compatibility.md#dmsspec) | 'dspdata.msspectrum' will be removed in a future release. Use 'periodogram' or 'pwelch' instead. |
| [`DPSPEC`](rules/compatibility.md#dpspec) | 'dspdata.pseudospectrum' will be removed in a future release. Use 'pmusic' or 'peig' instead. |
| [`DPPSD`](rules/compatibility.md#dppsd) | 'dspdata.psd' will be removed in a future release. Use 'pburg', 'pcov', 'peig', 'pmcov', 'pmtm', 'periodogram', or 'pwelch' instead. |
| [`DINLN`](rules/compatibility.md#dinln) | INLINE will be removed in a future release. Use anonymous functions instead. |
| [`DFCNCHK`](rules/compatibility.md#dfcnchk) | FCNCHK will be removed in a future release. Use anonymous functions instead. |
| [`MSYSTEM`](rules/compatibility.md#msystem) | matlab.system.System has been removed. Use matlab.System instead. |
| [`MATPOOL`](rules/compatibility.md#matpool) | 'matlabpool' has been removed. Use 'pool' instead. |
| [`OBJMPOOL`](rules/compatibility.md#objmpool) | 'MATLABPOOL' has been removed. Use 'PARPOOL' instead. |
| [`TREEDISP`](rules/compatibility.md#treedisp) | TREEDISP has been removed. Use ClassificationTree or RegressionTree VIEW methods instead. |
| [`TREEPRUNE`](rules/compatibility.md#treeprune) | TREEPRUNE has been removed. Use ClassificationTree or RegressionTree PRUNE methods instead. |
| [`TREETEST`](rules/compatibility.md#treetest) | 'treetest' has been removed. Use ClassificationTree or RegressionTree methods instead. |
| [`TREEVAL`](rules/compatibility.md#treeval) | TREEVAL has been removed. Use ClassificationTree or RegressionTree PREDICT methods instead. |
| [`TREEFIT`](rules/compatibility.md#treefit) | TREEFIT has been removed. Use fitctree or fitrtree instead. |
| [`FISGET`](rules/compatibility.md#fisget) | 'getfis' has been removed. Access FIS properties using dot notation instead. |
| [`FISM2M`](rules/compatibility.md#fism2m) | 'mf2mf' has been removed. Convert membership functions using dot notation on 'fismf' objects instead. |
| [`FISSET`](rules/compatibility.md#fisset) | 'setfis' has been removed. Set FIS properties using dot notation instead. |
| [`FISSHW`](rules/compatibility.md#fisshw) | 'showfis' has been removed. View FIS properties using dot notation instead. |
| [`WLGC`](rules/compatibility.md#wlgc) | 'wlanGeneratorConfig' has been removed. Use the name-value pair syntax of 'wlanWaveformGenerator' instead. |
| [`SMTHC`](rules/compatibility.md#smthc) | 'smithchart' has been removed. Use 'smithplot' instead. |
| [`WLRC`](rules/compatibility.md#wlrc) | 'wlanRecoveryConfig' has been removed. Instead, parameterize the function that accepts the 'wlanRecoveryConfig' object by using the name-value pair syntax. |
| [`JAVFM`](rules/compatibility.md#javfm) | 'JavaFrame' was undocumented and has been removed. There is no simple replacement for this. |
| [`JAVCT`](rules/compatibility.md#javct) | 'JavaContainer' was undocumented and has been removed. There is no simple replacement for this. |
| [`JAVCM`](rules/compatibility.md#javcm) | 'javacomponent' is undocumented and will be removed in a future release. There is no simple replacement for this. |
| [`COMMERRATE`](rules/compatibility.md#commerrate) | 'commtest.ErrorRate' has been removed. Use 'comm.ErrorRate' or BERTool instead. |
| [`TCRESULT`](rules/compatibility.md#tcresult) | 'testconsole.Results' has been removed. Use 'comm.ErrorRate' or BERTool instead. |
| [`OPGLI`](rules/compatibility.md#opgli) | 'opengl' has been removed. There is no simple replacement for this. |
| [`CNNCGD`](rules/compatibility.md#cnncgd) | 'cnncodegen' with default 'targetlib' as 'cudnn' has been removed. With appropriate code changes, use 'codegen' instead. |
| [`COMMSCOPEED`](rules/compatibility.md#commscopeed) | 'commscope.eyediagram' has been removed. For line plotting, use the eyediagram function. There is no simple replacement for histogram plotting and measurement analysis. |
| [`COMMED`](rules/compatibility.md#commed) | 'comm.EyeDiagram' has been removed. For line plotting, use the eyediagram function. There is no simple replacement for histogram plotting and measurement analysis. |
| [`MKRMT`](rules/compatibility.md#mkrmt) | 'makerefmat' has been removed. With appropriate code changes, construct a raster reference object using 'georefcells', 'georefpostings', 'georasterref', 'maprefcells', 'maprefpostings' or 'maprasterref' instead. |
| [`WFMRM`](rules/compatibility.md#wfmrm) | 'worldFileMatrixToRefmat' has been removed. With appropriate code changes, construct a raster reference object using 'georasterref' or 'maprasterref' instead. |
| [`RV2MAT`](rules/compatibility.md#rv2mat) | 'refvec2mat' has been removed. With appropriate code changes, construct a geographic raster reference object using 'refvecToGeoRasterReference' instead. |
| [`RM2VEC`](rules/compatibility.md#rm2vec) | 'refmat2vec' has been removed. With appropriate code changes, construct a geographic raster reference object using 'refvecToGeoRasterReference' instead. |
| [`SIZEM`](rules/compatibility.md#sizem) | 'sizem' has been removed. With appropriate code changes, use 'rastersize' property of a map raster reference object instead. |
| [`LIMIM`](rules/compatibility.md#limim) | 'limitm' has been removed. With appropriate code changes, use 'LatitudeLimits' and 'LongitudeLimits' properties of a geographic raster reference object instead. |
| [`MAPBX`](rules/compatibility.md#mapbx) | 'mapbbox' has been removed. With appropriate code changes, use 'XWorldLimits' and 'YWorldLimits' properties of a map raster reference object instead. |
| [`DBITMAX`](rules/compatibility.md#dbitmax) | 'bitmax' has been removed. Use 'flintmax' instead. |
| [`COLORDEF`](rules/compatibility.md#colordef) | 'colordef' will be removed in a future release. There is no simple replacement for this. |
| [`GRAYMON`](rules/compatibility.md#graymon) | 'graymon' will be removed in a future release. There is no simple replacement for this. |
| [`WHITEBG`](rules/compatibility.md#whitebg) | 'whitebg' will be removed in a future release. There is no simple replacement for this. |
| [`PRINTOPT`](rules/compatibility.md#printopt) | 'printopt' will be removed in a future release. There is no simple replacement for this. |
| [`HGEXPORT`](rules/compatibility.md#hgexport) | 'hgexport' will be removed in a future release. Use 'exportgraphics' or 'copygraphics' instead. |
| [`HANK2SYS`](rules/compatibility.md#hank2sys) | 'hank2sys' will be removed in a future release. There is no simple replacement for this. |
| [`HGSAVE`](rules/compatibility.md#hgsave) | 'hgsave' will be removed in a future release. Use 'savefig' instead. |
| [`HAND2STCT`](rules/compatibility.md#hand2stct) | 'handle2struct' will be removed in a future release. There is no simple replacement for this. |
| [`HILBIIR`](rules/compatibility.md#hilbiir) | 'hilbiir' will be removed in a future release. There is no simple replacement for this. |
| [`MOVIE2`](rules/compatibility.md#movie2) | 'movie2avi' will be removed in a future release. Use 'VideoWriter' instead. |
| [`SERIAL`](rules/compatibility.md#serial) | 'serial' will be removed in a future release. Use 'serialport' instead. |
| [`GPIB`](rules/compatibility.md#gpib) | 'gpib' will be removed in a future release. Use 'visadev' instead. |
| [`VISA`](rules/compatibility.md#visa) | 'visa' will be removed in a future release. Use 'visadev' instead. |
| [`UDPP`](rules/compatibility.md#udpp) | 'udp' will be removed in a future release. Use 'udpport' instead. |
| [`BLUTH`](rules/compatibility.md#bluth) | 'Bluetooth' (Instrument Control) will be removed in a future release. Use 'bluetooth' instead. |
| [`IMTOOL`](rules/compatibility.md#imtool) | 'imtool' will be removed in a future release. Use 'imageViewer' instead. |
| [`IMJAV`](rules/compatibility.md#imjav) | 'imjava' will be removed in a future release. There is no simple replacement for this. |
| [`DPOOL`](rules/compatibility.md#dpool) | 'parpool' legacy syntax will be removed in a future release. Use updated syntax instead. |
| [`MUPAD`](rules/compatibility.md#mupad) | 'mupad' will be removed in a future release. Use MATLAB Live Editor instead. |
| [`FBUILDER`](rules/compatibility.md#fbuilder) | 'filterBuilder' will be removed in a future release. Use 'designfilt' instead. |
| [`FDATOOL`](rules/compatibility.md#fdatool) | 'fdatool' will be removed in a future release. Use 'filterDesigner' instead. |
| [`WINTOOL`](rules/compatibility.md#wintool) | 'wintool' will be removed in a future release. Use 'windowDesigner' instead. |
| [`FVTOOL`](rules/compatibility.md#fvtool) | 'fvtool' will be removed in a future release. Use 'filterAnalyzer' instead. |
| [`FISNEW`](rules/compatibility.md#fisnew) | 'newfis' will be removed in a future release. Use 'mamfis' or 'sugfis' instead. |
| [`FISADV`](rules/compatibility.md#fisadv) | 'addvar' will be removed in a future release. Use 'addInput' or 'addOutput' instead. |
| [`FISRMV`](rules/compatibility.md#fisrmv) | 'rmvar' will be removed in a future release. Use 'removeInput' or 'removeOutput' instead. |
| [`FISRMF`](rules/compatibility.md#fisrmf) | 'rmmf' will be removed in a future release. Use 'removeMF' instead. |
| [`FISM2S`](rules/compatibility.md#fism2s) | 'mfedit' will be removed in a future release. There is no simple replacement for this. |
| [`FISPSR`](rules/compatibility.md#fispsr) | 'parsrule' will be removed in a future release. Use 'addRule' instead. |
| [`NPI2PI`](rules/compatibility.md#npi2pi) | 'npi2pi' will be removed in a future release. Use 'wrapToPi' instead. |
| [`NPI22PI`](rules/compatibility.md#npi22pi) | 'zero22pi' will be removed in a future release. Use 'wrapTo2Pi' instead. |
| [`CLASSREGTREE`](rules/compatibility.md#classregtree) | 'classregtree' will be removed in a future release. Use 'fitctree' or 'fitrtree' instead. |
| [`SVMCLASSIFY`](rules/compatibility.md#svmclassify) | 'svmclassify' will be removed in a future release. Use 'predict' on a trained SVM model instead. |
| [`SVMTRAIN`](rules/compatibility.md#svmtrain) | 'svmtrain' will be removed in a future release. Use 'fitcsvm' instead. |
| [`PRINCOMP`](rules/compatibility.md#princomp) | 'princomp' will be removed in a future release. Use 'pca' instead. |
| [`PROBDIST`](rules/compatibility.md#probdist) | 'ProbDist' will be removed in a future release. Use probability distribution objects instead. |
| [`PROBDISTPARAMETRIC`](rules/compatibility.md#probdistparametric) | 'ProbDistParametric' will be removed in a future release. Use probability distribution objects instead. |
| [`PROBDISTKERNEL`](rules/compatibility.md#probdistkernel) | 'ProbDistKernel' will be removed in a future release. Use probability distribution objects instead. |
| [`PROBDISTUNIVKERNEL`](rules/compatibility.md#probdistunivkernel) | 'ProbDistUnivKernel' will be removed in a future release. Use probability distribution objects instead. |
| [`PROBDISTUNIVPARAM`](rules/compatibility.md#probdistunivparam) | 'ProbDistUnivParam' will be removed in a future release. Use probability distribution objects instead. |
| [`FITNAIVEBAYES`](rules/compatibility.md#fitnaivebayes) | 'NaiveBayes.fit' will be removed in a future release. Use 'fitcnb' instead. |
| [`CAPABLE`](rules/compatibility.md#capable) | 'capable' will be removed in a future release. Use 'capability' instead. |
| [`EWMAPLOT`](rules/compatibility.md#ewmaplot) | 'ewmaplot' will be removed in a future release. Use 'controlchart' instead. |
| [`SCHART`](rules/compatibility.md#schart) | 'schart' will be removed in a future release. Use 'controlchart' instead. |
| [`XBARPLOT`](rules/compatibility.md#xbarplot) | 'xbarplot' will be removed in a future release. Use 'controlchart' instead. |
| [`RANDSD`](rules/compatibility.md#randsd) | 'randseed' will be removed in a future release. Use 'rng' instead. |
| [`DRNDINT`](rules/compatibility.md#drndint) | 'randint' will be removed in a future release. Use 'randi' instead. |
| [`ISGLOB`](rules/compatibility.md#isglob) | 'isglobal' will be removed in a future release. There is no simple replacement for this. |
| [`DGRAPHICSVER`](rules/compatibility.md#dgraphicsver) | 'graphicsversion' will be removed in a future release. There is no simple replacement for this. |
| [`DNOANI`](rules/compatibility.md#dnoani) | 'noanimate' will be removed in a future release. There is no simple replacement for this. |
| [`EPSM`](rules/compatibility.md#epsm) | 'epsm' will be removed in a future release. Use 'eps' instead. |
| [`DEXIFRD`](rules/compatibility.md#dexifrd) | 'exifread' will be removed in a future release. Use 'imfinfo' instead. |
| [`HDFGD`](rules/compatibility.md#hdfgd) | 'hdfgd' will be removed in a future release. There is no simple replacement for this. |
| [`HDFSD`](rules/compatibility.md#hdfsd) | 'hdfsd' will be removed in a future release. There is no simple replacement for this. |
| [`HDFSW`](rules/compatibility.md#hdfsw) | 'hdfsw' will be removed in a future release. There is no simple replacement for this. |
| [`HDFTL`](rules/compatibility.md#hdftl) | 'hdftl' will be removed in a future release. There is no simple replacement for this. |
| [`HYPERCUBE`](rules/compatibility.md#hypercube) | 'hypercube' will be removed in a future release. Use 'blockedImage' instead. |
| [`BIGIMAGE`](rules/compatibility.md#bigimage) | 'bigimage' will be removed in a future release. Use 'blockedImage' instead. |
| [`BIGIMAGEDS`](rules/compatibility.md#bigimageds) | 'bigimageDatastore' will be removed in a future release. Use 'blockedImageDatastore' instead. |
| [`LABELVOL`](rules/compatibility.md#labelvol) | 'labelvolshow' will be removed in a future release. Use 'labeloverlay3' instead. |
| [`COMPATVOL`](rules/compatibility.md#compatvol) | 'volshow' will be removed in a future release. Use 'viewer3d' with 'volshow' instead. |
| [`DEMLC`](rules/compatibility.md#demlc) | 'emlc' will be removed in a future release. Use 'codegen' instead. |
| [`DEMLMEX`](rules/compatibility.md#demlmex) | 'emlmex' will be removed in a future release. Use 'codegen' instead. |
| [`COMMBI`](rules/compatibility.md#commbi) | 'biterr' (legacy) will be removed in a future release. Use 'comm.ErrorRate' instead. |
| [`PNZOM`](rules/compatibility.md#pnzom) | 'panzoom' will be removed in a future release. Use 'zoom' and 'pan' instead. |
| [`SPTL`](rules/compatibility.md#sptl) | 'sptool' will be removed in a future release. Use 'signalAnalyzer' instead. |
| [`NDWT`](rules/compatibility.md#ndwt) | 'ndwt' will be removed in a future release. Use 'waveletTransform3' instead. |
| [`INDWT`](rules/compatibility.md#indwt) | 'indwt' will be removed in a future release. Use 'waveletTransform3' inverse instead. |
| [`NDWT2`](rules/compatibility.md#ndwt2) | 'ndwt2' will be removed in a future release. There is no simple replacement for this. |
| [`INDWT2`](rules/compatibility.md#indwt2) | 'indwt2' will be removed in a future release. There is no simple replacement for this. |
| [`WEBMP`](rules/compatibility.md#webmp) | 'webmap' will be removed in a future release. Use geographic map visualizations instead. |
| [`SERLL`](rules/compatibility.md#serll) | 'seriallist' will be removed in a future release. Use 'serialportlist' instead. |
| [`RNG2BW`](rules/compatibility.md#rng2bw) | 'range2bw' will be removed in a future release. Use 'rangeres2bw' instead. |
| [`BW2RNG`](rules/compatibility.md#bw2rng) | 'bw2range' will be removed in a future release. Use 'bw2rangeres' instead. |
| [`DCSHELP`](rules/compatibility.md#dcshelp) | 'docsearch' will be removed in a future release. There is no simple replacement for this. |
| [`DFIGFLAG`](rules/compatibility.md#dfigflag) | 'figflag' will be removed in a future release. There is no simple replacement for this. |
| [`DPOPUP`](rules/compatibility.md#dpopup) | 'popupstr' will be removed in a future release. There is no simple replacement for this. |
| [`ACTXC`](rules/compatibility.md#actxc) | 'actxcontrol' will be removed in a future release. There is no simple replacement for this. |
| [`ACTXL`](rules/compatibility.md#actxl) | 'actxcontrollist' will be removed in a future release. There is no simple replacement for this. |
| [`ACTXS`](rules/compatibility.md#actxs) | 'actxcontrolselect' will be removed in a future release. There is no simple replacement for this. |
| [`SOAPM`](rules/compatibility.md#soapm) | 'createSoapMessage' will be removed in a future release. Use 'matlab.net.http' instead. |
| [`SOAPS`](rules/compatibility.md#soaps) | 'callSoapService' will be removed in a future release. Use 'matlab.net.http' instead. |
| [`SOAPR`](rules/compatibility.md#soapr) | 'parseSoapResponse' will be removed in a future release. Use 'matlab.net.http' instead. |
| [`SOAPC`](rules/compatibility.md#soapc) | 'createClassFromWsdl' will be removed in a future release. Use 'matlab.net.http' instead. |
| [`VREDT`](rules/compatibility.md#vredt) | 'vredit' will be removed in a future release. There is no simple replacement for this. |
| [`VRPLY`](rules/compatibility.md#vrply) | 'vrplay' will be removed in a future release. There is no simple replacement for this. |
| [`VRCNVS`](rules/compatibility.md#vrcnvs) | 'vrcanvas' will be removed in a future release. There is no simple replacement for this. |
| [`VRFIG`](rules/compatibility.md#vrfig) | 'vrfigure' will be removed in a future release. There is no simple replacement for this. |
| [`VRVIEW`](rules/compatibility.md#vrview) | 'vrview' will be removed in a future release. There is no simple replacement for this. |
| [`VRWRLD`](rules/compatibility.md#vrwrld) | 'vrworld' will be removed in a future release. There is no simple replacement for this. |
| [`VRNDE`](rules/compatibility.md#vrnde) | 'vrnode' will be removed in a future release. There is no simple replacement for this. |
| [`VRJOYSTK`](rules/compatibility.md#vrjoystk) | 'vrjoystick' will be removed in a future release. There is no simple replacement for this. |
| [`VRSPCMOUSE`](rules/compatibility.md#vrspcmouse) | 'vrspacemouse' will be removed in a future release. There is no simple replacement for this. |
| [`SLRTBNCH`](rules/compatibility.md#slrtbnch) | 'slrtbench' will be removed in a future release. There is no simple replacement for this. |
| [`RCSIIR`](rules/compatibility.md#rcsiir) | 'rcosine' (IIR) will be removed in a future release. Use 'rcosdesign' instead. |
| [`RCSFIR`](rules/compatibility.md#rcsfir) | 'rcosine' (FIR) will be removed in a future release. Use 'rcosdesign' instead. |
| [`G2B`](rules/compatibility.md#g2b) | 'gray2bin' will be removed in a future release. There is no simple replacement for this. |
| [`B2G`](rules/compatibility.md#b2g) | 'bin2gray' will be removed in a future release. There is no simple replacement for this. |
| [`EYESCOPE`](rules/compatibility.md#eyescope) | 'eyediagram' will be removed in a future release. Use 'comm.EyeDiagram' instead. |
| [`DLDPCENC`](rules/compatibility.md#dldpcenc) | 'fec.ldpcenc' will be removed in a future release. Use 'comm.LDPCEncoder' instead. |
| [`DLDPCDEC`](rules/compatibility.md#dldpcdec) | 'fec.ldpcdec' will be removed in a future release. Use 'comm.LDPCDecoder' instead. |
| [`WEIBCDF`](rules/compatibility.md#weibcdf) | 'weibcdf' will be removed in a future release. Use 'wblcdf' instead. |
| [`WEIBFIT`](rules/compatibility.md#weibfit) | 'weibfit' will be removed in a future release. Use 'wblfit' instead. |
| [`WEIBINV`](rules/compatibility.md#weibinv) | 'weibinv' will be removed in a future release. Use 'wblinv' instead. |
| [`WEIBLIKE`](rules/compatibility.md#weiblike) | 'weiblike' will be removed in a future release. Use 'wbllike' instead. |
| [`WEIBPDF`](rules/compatibility.md#weibpdf) | 'weibpdf' will be removed in a future release. Use 'wblpdf' instead. |
| [`WEIBPLOT`](rules/compatibility.md#weibplot) | 'weibplot' will be removed in a future release. Use 'wblplot' instead. |
| [`WEIBRND`](rules/compatibility.md#weibrnd) | 'weibrnd' will be removed in a future release. Use 'wblrnd' instead. |
| [`WEIBSTAT`](rules/compatibility.md#weibstat) | 'weibstat' will be removed in a future release. Use 'wblstat' instead. |
| [`OPCSERVER`](rules/compatibility.md#opcserver) | 'opcserverinfo' will be removed in a future release. There is no simple replacement for this. |
| [`OPCDA`](rules/compatibility.md#opcda) | 'opcda' will be removed in a future release. Use 'opcua' instead. |
| [`OPCFIND`](rules/compatibility.md#opcfind) | 'opcfind' will be removed in a future release. There is no simple replacement for this. |
| [`OPCRESET`](rules/compatibility.md#opcreset) | 'opcreset' will be removed in a future release. There is no simple replacement for this. |
| [`OPCHELP`](rules/compatibility.md#opchelp) | 'opchelp' will be removed in a future release. There is no simple replacement for this. |
| [`DSPBQF`](rules/compatibility.md#dspbqf) | 'dsp.BiquadFilter' will be removed in a future release. Use 'dsp.SOSFilter' instead. |
| [`DSPLPC`](rules/compatibility.md#dsplpc) | 'dsp.LPC' will be removed in a future release. Use 'lpc' instead. |
| [`DSPAVA`](rules/compatibility.md#dspava) | 'dsp.ArrayVectorAdder' will be removed in a future release. Use built-in addition instead. |
| [`DSPAVS`](rules/compatibility.md#dspavs) | 'dsp.ArrayVectorSubtractor' will be removed in a future release. Use built-in subtraction instead. |
| [`DSPAVM`](rules/compatibility.md#dspavm) | 'dsp.ArrayVectorMultiplier' will be removed in a future release. Use built-in multiplication instead. |
| [`DSPAVD`](rules/compatibility.md#dspavd) | 'dsp.ArrayVectorDivider' will be removed in a future release. Use built-in division instead. |
| [`DSPCTR`](rules/compatibility.md#dspctr) | 'dsp.Counter' will be removed in a future release. There is no simple replacement for this. |
| [`DSPKFT`](rules/compatibility.md#dspkft) | 'dsp.KalmanFilter' will be removed in a future release. There is no simple replacement for this. |
| [`DSPNORM`](rules/compatibility.md#dspnorm) | 'dsp.Normalizer' will be removed in a future release. Use 'normalize' instead. |
| [`DSPAUDIOREC`](rules/compatibility.md#dspaudiorec) | 'dsp.AudioRecorder' will be removed in a future release. Use 'audioDeviceReader' instead. |
| [`DSPAUDIOPLAY`](rules/compatibility.md#dspaudioplay) | 'dsp.AudioPlayer' will be removed in a future release. Use 'audioDeviceWriter' instead. |
| [`DSPBUFFER`](rules/compatibility.md#dspbuffer) | 'dsp.Buffer' will be removed in a future release. Use 'dsp.AsyncBuffer' instead. |
| [`DSPHIST`](rules/compatibility.md#dsphist) | 'dsp.Histogram' will be removed in a future release. Use 'histogram' instead. |
| [`DSPMAX`](rules/compatibility.md#dspmax) | 'dsp.Maximum' will be removed in a future release. Use 'movmax' or 'max' instead. |
| [`DSPMIN`](rules/compatibility.md#dspmin) | 'dsp.Minimum' will be removed in a future release. Use 'movmin' or 'min' instead. |
| [`DSPMEAN`](rules/compatibility.md#dspmean) | 'dsp.Mean' will be removed in a future release. Use 'movmean' or 'mean' instead. |
| [`DSPMEDIAN`](rules/compatibility.md#dspmedian) | 'dsp.Median' will be removed in a future release. Use 'movmedian' or 'median' instead. |
| [`DSPRMS`](rules/compatibility.md#dsprms) | 'dsp.RMS' will be removed in a future release. Use 'rms' instead. |
| [`DSPSTD`](rules/compatibility.md#dspstd) | 'dsp.StandardDeviation' will be removed in a future release. Use 'movstd' or 'std' instead. |
| [`DSPVAR`](rules/compatibility.md#dspvar) | 'dsp.Variance' will be removed in a future release. Use 'movvar' or 'var' instead. |
| [`DSPCUMPROD`](rules/compatibility.md#dspcumprod) | 'dsp.CumulativeProduct' will be removed in a future release. Use 'cumprod' instead. |
| [`DSPCUMSUM`](rules/compatibility.md#dspcumsum) | 'dsp.CumulativeSum' will be removed in a future release. Use 'cumsum' instead. |
| [`DSPINTERP`](rules/compatibility.md#dspinterp) | 'dsp.Interpolator' will be removed in a future release. Use 'interp1' instead. |
| [`DSPCONV`](rules/compatibility.md#dspconv) | 'dsp.Convolver' will be removed in a future release. Use 'conv' instead. |
| [`DSPAUTOCORR`](rules/compatibility.md#dspautocorr) | 'dsp.Autocorrelator' will be removed in a future release. Use 'xcorr' instead. |
| [`DSPXCORR`](rules/compatibility.md#dspxcorr) | 'dsp.Crosscorrelator' will be removed in a future release. Use 'xcorr' instead. |
| [`DSPLDL`](rules/compatibility.md#dspldl) | 'dsp.LDLFactor' will be removed in a future release. Use 'ldl' instead. |
| [`DSPLU`](rules/compatibility.md#dsplu) | 'dsp.LUFactor' will be removed in a future release. Use 'lu' instead. |
| [`DSPLEVINSON`](rules/compatibility.md#dsplevinson) | 'dsp.LevinsonSolver' will be removed in a future release. Use 'levinson' instead. |
| [`DSPDELAYLINE`](rules/compatibility.md#dspdelayline) | 'dsp.DelayLine' will be removed in a future release. Use 'dsp.AsyncBuffer' instead. |
| [`DSPWIN`](rules/compatibility.md#dspwin) | 'dsp.Window' will be removed in a future release. Use 'window functions' directly instead. |
| [`DSPSA`](rules/compatibility.md#dspsa) | 'dsp.SpectrumAnalyzer' scope syntax will be removed in a future release. Use 'spectrumAnalyzer' instead. |
| [`CMLLD`](rules/compatibility.md#cmlld) | 'ClassificationModel.logLoss' will be removed in a future release. There is no simple replacement for this. |
| [`CMLSV`](rules/compatibility.md#cmlsv) | 'ClassificationModel.logSV' will be removed in a future release. There is no simple replacement for this. |
| [`SESSION`](rules/compatibility.md#session) | 'daq.createSession' will be removed in a future release. Use 'daqlist' and 'daq' interface instead. |
| [`DRYICE`](rules/compatibility.md#dryice) | 'dryice' will be removed in a future release. There is no simple replacement for this. |
| [`PERFREF`](rules/compatibility.md#perfref) | 'perfcurve' old syntax will be removed in a future release. Use updated syntax instead. |
| [`YOLOV`](rules/compatibility.md#yolov) | 'yolov2ObjectDetector' will be removed in a future release. Use 'yoloxObjectDetector' instead. |
| [`DDTRD`](rules/compatibility.md#ddtrd) | 'dtree' will be removed in a future release. There is no simple replacement for this. |
| [`IMPIVD`](rules/compatibility.md#impivd) | Malformed import argument VAR_NAME will not be supported in a future release. |
| [`IMPKEY`](rules/compatibility.md#impkey) | Importing VAR_NAME will not be supported in a future release because VAR_NAME is a reserved word. |
| [`REDEFGI`](rules/compatibility.md#redefgi) | Declaring an input or output variable to be global might not be supported in a future release. |
| [`REDEFGG`](rules/compatibility.md#redefgg) | Declaring a variable to be global more than once might not be supported in a future release. |
| [`MCPDC`](rules/compatibility.md#mcpdc) | Specifying both the 'Constant' and 'Dependent' attributes on the same property is not supported. |
| [`NOV6`](rules/compatibility.md#nov6) | 'v6' will be removed in a future release. There is no simple replacement for this. |
| [`V6ON`](rules/compatibility.md#v6on) | USEV6PLOTAPI('on') will be removed in a future release. There is no simple replacement for this. |
| [`PSTAT`](rules/compatibility.md#pstat) | The 'Static' attribute on properties has been removed. Use the 'Constant' attribute instead. |
| [`FGREN`](rules/compatibility.md#fgren) | 'Renderer' will be removed in a future release and currently has no effect. There is no simple replacement for this. |
| [`FGREM`](rules/compatibility.md#fgrem) | 'RendererMode' will be removed in a future release and currently has no effect. There is no simple replacement for this. |
| [`FROPT`](rules/compatibility.md#fropt) | '-dill' has been removed. Use Encapsulated PostScript instead. |
| [`FROPTX`](rules/compatibility.md#froptx) | '-adobecset' has been removed. There is no simple replacement for this. |
| [`DSPIDF`](rules/compatibility.md#dspidf) | 'DirectFeedthrough' property of 'dsp.VariableIntegerDelay' class has been removed. |
| [`DSPFDF`](rules/compatibility.md#dspfdf) | 'DirectFeedthrough' property of 'dsp.VariableFractionalDelay' class has been removed. |
| [`ATVIZW`](rules/compatibility.md#atvizw) | The 'Visible' attribute has been removed. Use the '~Hidden' attribute instead or omit the attribute since 'Hidden' is false by default. |
| [`MCGCP`](rules/compatibility.md#mcgcp) | Defining a get method for a constant property is not supported. |
| [`MCATP`](rules/compatibility.md#mcatp) | Using an @ sign to specify a class property restriction is unsupported and has been removed. Use property validation syntax instead. |
| [`NCHKNO`](rules/compatibility.md#nchkno) | NARGOUTCHK using more than two inputs will be removed in a future release. There is no simple replacement for this. |
| [`HESST`](rules/compatibility.md#hesst) | 'InitialHessType' has been removed. There is no simple replacement for this. |
| [`HESSM`](rules/compatibility.md#hessm) | 'InitialHessMatrix' has been removed. There is no simple replacement for this. |
| [`BUFSIZE`](rules/compatibility.md#bufsize) | Option 'Bufsize' has been removed. Manual buffering in 'textscan' is no longer needed. |
| [`FEGLO`](rules/compatibility.md#feglo) | 'global' has been removed. There is no simple replacement for this. |
| [`SMPLMODE`](rules/compatibility.md#smplmode) | The property 'FrameBasedProcessing' has been removed. |
| [`RESOU`](rules/compatibility.md#resou) | 'resources' is a reserved folder. Running MATLAB files located in a folder named 'resources' is not supported. |
| [`TTSMP`](rules/compatibility.md#ttsmp) | 'SamplingRate' has been removed. Use 'SampleRate' instead. |
| [`FINSI`](rules/compatibility.md#finsi) | Support for 'inputname' in a script has been removed. |
| [`FINSNI`](rules/compatibility.md#finsni) | Support for 'nargin' in a script has been removed. |
| [`FINSNO`](rules/compatibility.md#finsno) | Support for 'nargout' in a script has been removed. |
| [`DISGVER`](rules/compatibility.md#disgver) | 'matlab.graphics.internal.isGraphicsVersion1' has been removed. Use 'verLessThan('matlab','8.4.0')' instead. |
| [`DNDLA`](rules/compatibility.md#dndla) | 'discard' has been removed. There is no simple replacement for this. |
| [`EITYCN`](rules/compatibility.md#eitycn) | feature('EightyColumns') and feature('EightyColumns', VALUE) are unsupported and have been removed. With appropriate code changes, use 'settings' object instead. |
| [`REPUDD`](rules/compatibility.md#repudd) | Classes defined using schema.m files are no longer supported. Use MATLAB Classes defined using the classdef keyword instead. |
| [`RTWHWDR`](rules/compatibility.md#rtwhwdr) | 'RTW.HWDeviceRegistry' is unsupported and has been removed. The replacement strategy can be found in MATLAB documentation. |
| [`INVHCRM`](rules/compatibility.md#invhcrm) | The 'InvertHardCopy' property will be removed in a future release and currently has no effect. |
| [`DINVHCRM`](rules/compatibility.md#dinvhcrm) | The 'defaultFigureInvertHardCopy' setting will be removed in a future release and currently has no effect. |
| [`CHKGR`](rules/compatibility.md#chkgr) | The 'CheckGradients' option has been removed from 'optimoptions'. With appropriate code changes, use the 'checkGradients' function instead. |
| [`LINPROGS`](rules/compatibility.md#linprogs) | 'simplex' algorithm has been removed. With appropriate code changes, set 'Algorithm' value to 'interior-point' or 'dual-simplex' instead. |
| [`LSRET`](rules/compatibility.md#lsret) | 'LaserReturns' has been removed. Use 'LaserReturn' instead, which is a direct replacement. |
| [`RAYNR`](rules/compatibility.md#raynr) | Input argument 'NumReflections' has been removed. Use 'MaxNumReflections' property of a ray tracing propagation model object instead. |
| [`JAPIMATHWORKS`](rules/compatibility.md#japimathworks) | 'com.mathworks' namespace and sub namespaces will be removed in a future release. There is no simple replacement for this. |
| [`IMCLASS`](rules/compatibility.md#imclass) | In a future release, 'ismethod' will treat a string or character vector in its first input as a 'string' or 'char' class object. |
| [`WEBREMOVE`](rules/compatibility.md#webremove) | The 'web' function does not return a handle or URL for pages that open in the system browser. Use 'stat = web(___, '-browser')' instead. |
| [`TCPC`](rules/compatibility.md#tcpc) | 'tcpip' with 'client' as a 'NetworkRole' will be removed in a future release. With appropriate code changes, use 'tcpclient' instead. |
| [`TCPS`](rules/compatibility.md#tcps) | 'tcpip' with 'server' as a 'NetworkRole' will be removed in a future release. With appropriate code changes, use 'tcpserver' instead. |
| [`QAMDEPM`](rules/compatibility.md#qamdepm) | 'qammod' no longer accepts the initial phase of a signal. |
| [`QAMDEPD`](rules/compatibility.md#qamdepd) | 'qamdemod' no longer accepts the initial phase of a signal. |
| [`GETERR`](rules/compatibility.md#geterr) | The 'ErrorMessage' property has been removed. At the command line, use 'MException.last' instead. |
| [`SETERR`](rules/compatibility.md#seterr) | The 'ErrorMessage' property has been removed. There is no simple replacement for this. |
| [`MAPVW`](rules/compatibility.md#mapvw) | 'mapview' will be removed in a future release. Use geographic or map axes instead. |
| [`MAPTOOL`](rules/compatibility.md#maptool) | 'maptool' will be removed in a future release. There is no simple replacement for this. |
| [`DFEATUREPARAM1`](rules/compatibility.md#dfeatureparam1) | 'UseHG2' has been removed. With appropriate code changes, use '~verLessThan('matlab','8.4.0')' instead. |
| [`DFEATUREPARAM2`](rules/compatibility.md#dfeatureparam2) | 'HGUsingMATLABClasses' has been removed. With appropriate code changes, use '~verLessThan('matlab','8.4.0')' instead. |
| [`DILEVAL`](rules/compatibility.md#dileval) | 'dicomLookupEval' will be removed in a future release. There is no simple replacement for this. |
| [`DSPPEAKS`](rules/compatibility.md#dsppeaks) | 'dsp.PeakFinder' will be removed in a future release. Use 'findpeaks' instead. |
| [`DSPPEAK2PEAK`](rules/compatibility.md#dsppeak2peak) | 'dsp.PeakToPeak' will be removed in a future release. Use 'peak2peak' instead. |
| [`DSP_LINKS`](rules/compatibility.md#dsplinks) | 'dsp.SignalSource' will be removed in a future release. There is no simple replacement for this. |
| [`MCASCADE`](rules/compatibility.md#mcascade) | 'mfilt.cascade' will be removed in a future release. Use 'dsp.FilterCascade' instead. |
| [`FDESPARAMEQ`](rules/compatibility.md#fdesparameq) | 'fdesign.parameq' will be removed in a future release. Use 'designParamEQ' instead. |
| [`FDESOCTAVE`](rules/compatibility.md#fdesoctave) | 'fdesign.octave' will be removed in a future release. Use 'designOctaveFilter' instead. |
| [`FDESWEIGHT`](rules/compatibility.md#fdesweight) | 'fdesign.arbmagnphase' will be removed in a future release. Use 'designfilt' instead. |
| [`AFLMS`](rules/compatibility.md#aflms) | 'adaptfilt.lms' will be removed in a future release. Use 'dsp.LMSFilter' instead. |
| [`AFNLMS`](rules/compatibility.md#afnlms) | 'adaptfilt.nlms' will be removed in a future release. Use 'dsp.LMSFilter' with NLMS algorithm instead. |
| [`AFRLS`](rules/compatibility.md#afrls) | 'adaptfilt.rls' will be removed in a future release. Use 'dsp.RLSFilter' instead. |
| [`AFBLMS`](rules/compatibility.md#afblms) | 'adaptfilt.blms' will be removed in a future release. Use 'dsp.BlockLMSFilter' instead. |
| [`DNANMEAN`](rules/compatibility.md#dnanmean) | 'nanmean' from Statistics Toolbox is not recommended. Use 'mean' with 'omitnan' option instead. |
| [`DNANSTD`](rules/compatibility.md#dnanstd) | 'nanstd' from Statistics Toolbox is not recommended. Use 'std' with 'omitnan' option instead. |
| [`DNANVAR`](rules/compatibility.md#dnanvar) | 'nanvar' from Statistics Toolbox is not recommended. Use 'var' with 'omitnan' option instead. |
| [`DNANMEDIAN`](rules/compatibility.md#dnanmedian) | 'nanmedian' from Statistics Toolbox is not recommended. Use 'median' with 'omitnan' option instead. |
| [`DNANMIN`](rules/compatibility.md#dnanmin) | 'nanmin' from Statistics Toolbox is not recommended. Use 'min' with 'omitnan' option instead. |
| [`DNANMAX`](rules/compatibility.md#dnanmax) | 'nanmax' from Statistics Toolbox is not recommended. Use 'max' with 'omitnan' option instead. |
| [`DNANSUM`](rules/compatibility.md#dnansum) | 'nansum' from Statistics Toolbox is not recommended. Use 'sum' with 'omitnan' option instead. |
| [`DNANCOV`](rules/compatibility.md#dnancov) | 'nancov' from Statistics Toolbox is not recommended. Use covariance computation with 'omitrows' option instead. |
| [`DWAVREAD`](rules/compatibility.md#dwavread) | 'wavread' has been removed. Use 'audioread' instead. |
| [`DWAVWRITE`](rules/compatibility.md#dwavwrite) | 'wavwrite' has been removed. Use 'audiowrite' instead. |
| [`DAUREAD`](rules/compatibility.md#dauread) | 'auread' has been removed. Use 'audioread' instead. |
| [`DAUWRITE`](rules/compatibility.md#dauwrite) | 'auwrite' has been removed. Use 'audiowrite' instead. |
| [`DQUAD2`](rules/compatibility.md#dquad2) | 'quad' is not recommended. Use 'integral' instead. |
| [`DQUADL2`](rules/compatibility.md#dquadl2) | 'quadl' is not recommended. Use 'integral' instead. |
| [`DQUADV2`](rules/compatibility.md#dquadv2) | 'quadv' is not recommended. Use 'integral' with 'ArrayValued' option instead. |
| [`DDBLQD2`](rules/compatibility.md#ddblqd2) | 'dblquad' is not recommended. Use 'integral2' instead. |
| [`DTRIQD2`](rules/compatibility.md#dtriqd2) | 'triplequad' is not recommended. Use 'integral3' instead. |
| [`DEZPLOT`](rules/compatibility.md#dezplot) | 'ezplot' is not recommended. Use 'fplot' instead. |
| [`DEZMESH`](rules/compatibility.md#dezmesh) | 'ezmesh' is not recommended. Use 'fmesh' instead. |
| [`DEZSURF`](rules/compatibility.md#dezsurf) | 'ezsurf' is not recommended. Use 'fsurf' instead. |
| [`DEZCONTOUR`](rules/compatibility.md#dezcontour) | 'ezcontour' is not recommended. Use 'fcontour' instead. |
| [`DEZPOLAR`](rules/compatibility.md#dezpolar) | 'ezpolar' is not recommended. Use 'polarplot' instead. |
| [`DEZPLOT3`](rules/compatibility.md#dezplot3) | 'ezplot3' is not recommended. Use 'fplot3' instead. |
| [`DEZSURFC`](rules/compatibility.md#dezsurfc) | 'ezsurfc' is not recommended. Use 'fsurf' instead. |
| [`DEZMESHC`](rules/compatibility.md#dezmeshc) | 'ezmeshc' is not recommended. Use 'fmesh' instead. |
| [`DEZCONTOURF`](rules/compatibility.md#dezcontourf) | 'ezcontourf' is not recommended. Use 'fcontour' instead. |
| [`DPLOTYY`](rules/compatibility.md#dplotyy) | 'plotyy' is not recommended. Use 'yyaxis' instead. |
| [`DPOLAR`](rules/compatibility.md#dpolar) | 'polar' is not recommended. Use 'polarplot' instead. |
| [`DCOMPASS`](rules/compatibility.md#dcompass) | 'compass' is not recommended. Use 'polarplot' instead. |
| [`DROSE`](rules/compatibility.md#drose) | 'rose' is not recommended. Use 'polarhistogram' instead. |
| [`DHIST`](rules/compatibility.md#dhist) | 'hist' is not recommended. Use 'histogram' instead. |
| [`DHISTC`](rules/compatibility.md#dhistc) | 'histc' is not recommended. Use 'histcounts' instead. |
| [`DCAXIS`](rules/compatibility.md#dcaxis) | 'caxis' is not recommended. Use 'clim' instead. |
| [`DHOLDALL`](rules/compatibility.md#dholdall) | 'hold all' is not recommended. Use 'hold on' instead. |
| [`DCSVREAD`](rules/compatibility.md#dcsvread) | 'csvread' is not recommended. Use 'readmatrix' instead. |
| [`DCSVWRITE`](rules/compatibility.md#dcsvwrite) | 'csvwrite' is not recommended. Use 'writematrix' instead. |
| [`DDLMREAD`](rules/compatibility.md#ddlmread) | 'dlmread' is not recommended. Use 'readmatrix' instead. |
| [`DDLMWRITE`](rules/compatibility.md#ddlmwrite) | 'dlmwrite' is not recommended. Use 'writematrix' instead. |
| [`DXLSREAD`](rules/compatibility.md#dxlsread) | 'xlsread' is not recommended. Use 'readtable', 'readmatrix', or 'readcell' instead. |
| [`DXLSWRITE`](rules/compatibility.md#dxlswrite) | 'xlswrite' is not recommended. Use 'writetable', 'writematrix', or 'writecell' instead. |
| [`DTEXTREAD`](rules/compatibility.md#dtextread) | 'textread' has been removed. Use 'textscan' instead. |
| [`DSTRREAD`](rules/compatibility.md#dstrread) | 'strread' has been removed. Use 'textscan' instead. |
| [`DURLREAD`](rules/compatibility.md#durlread) | 'urlread' is not recommended. Use 'webread' instead. |
| [`DURLWRITE`](rules/compatibility.md#durlwrite) | 'urlwrite' is not recommended. Use 'websave' instead. |
| [`DH5READ`](rules/compatibility.md#dh5read) | 'hdf5read' has been removed. Use 'h5read' instead. |
| [`DH5WRITE`](rules/compatibility.md#dh5write) | 'hdf5write' has been removed. Use 'h5write' instead. |
| [`DH5INFO`](rules/compatibility.md#dh5info) | 'hdf5info' has been removed. Use 'h5info' instead. |
| [`DIM2BW`](rules/compatibility.md#dim2bw) | 'im2bw' is not recommended. Use 'imbinarize' instead. |
| [`DIMFREEHAND`](rules/compatibility.md#dimfreehand) | 'imfreehand' is not recommended. Use 'drawfreehand' instead. |
| [`DIMRECT`](rules/compatibility.md#dimrect) | 'imrect' is not recommended. Use 'drawrectangle' instead. |
| [`DIMLINE`](rules/compatibility.md#dimline) | 'imline' is not recommended. Use 'drawline' instead. |
| [`DIMPOINT`](rules/compatibility.md#dimpoint) | 'impoint' is not recommended. Use 'drawpoint' instead. |
| [`DIMPOLY`](rules/compatibility.md#dimpoly) | 'impoly' is not recommended. Use 'drawpolygon' instead. |
| [`DIMELLIPSE`](rules/compatibility.md#dimellipse) | 'imellipse' is not recommended. Use 'drawellipse' instead. |
| [`DROIFILL`](rules/compatibility.md#droifill) | 'roifill' is not recommended. Use 'regionfill' instead. |
| [`DDATASET`](rules/compatibility.md#ddataset) | 'dataset' is not recommended. Use 'table' instead. |
| [`DNOMINAL`](rules/compatibility.md#dnominal) | 'nominal' has been removed. Use 'categorical' instead. |
| [`DORDINAL`](rules/compatibility.md#dordinal) | 'ordinal' has been removed. Use 'categorical' with 'Ordinal' option instead. |
| [`DDATENUM`](rules/compatibility.md#ddatenum) | 'datenum' is not recommended. Use 'datetime' instead. |
| [`DDATESTR`](rules/compatibility.md#ddatestr) | 'datestr' is not recommended. Use 'string' or 'char' on datetime objects instead. |
| [`DDATEVEC`](rules/compatibility.md#ddatevec) | 'datevec' is not recommended. Use datetime properties (Year, Month, Day, etc.) instead. |
| [`DCLOCK`](rules/compatibility.md#dclock) | 'clock' is not recommended. Use 'datetime(\ |
| [`DDATE`](rules/compatibility.md#ddate) | 'date' is not recommended. Use 'datetime(\ |
| [`DNOW`](rules/compatibility.md#dnow) | 'now' is not recommended. Use 'datetime(\ |
| [`DTODAY`](rules/compatibility.md#dtoday) | 'today' (datenum) is not recommended. Use 'datetime(\ |
| [`DTIC`](rules/compatibility.md#dtic) | Using 'tic'/'toc' for profiling is not recommended. Use 'timeit' for accurate timing. |
| [`DDEBLANK`](rules/compatibility.md#ddeblank) | 'deblank' is not recommended. Use 'strtrim' or 'strip' instead. |
| [`DFINDSTR`](rules/compatibility.md#dfindstr) | 'findstr' has been removed. Use 'strfind' or 'contains' instead. |
| [`DSTRMATCH`](rules/compatibility.md#dstrmatch) | 'strmatch' has been removed. Use 'startsWith', 'matches', or 'strcmp' instead. |
| [`DSTRVCAT`](rules/compatibility.md#dstrvcat) | 'strvcat' has been removed. Use 'char' or string arrays instead. |
| [`DGENVARNAME`](rules/compatibility.md#dgenvarname) | 'genvarname' has been removed. Use 'matlab.lang.makeValidName' instead. |
| [`DGUIDE`](rules/compatibility.md#dguide) | 'guide' (GUIDE) is not recommended. Use App Designer instead. |
| [`DINPUTDLG`](rules/compatibility.md#dinputdlg) | 'inputdlg' is not recommended for new apps. Use App Designer UI components instead. |
| [`DWARNDLG`](rules/compatibility.md#dwarndlg) | 'warndlg' is not recommended. Use 'uialert' instead. |
| [`DERRORDLG`](rules/compatibility.md#derrordlg) | 'errordlg' is not recommended. Use 'uialert' instead. |
| [`DMSGBOX`](rules/compatibility.md#dmsgbox) | 'msgbox' is not recommended. Use 'uialert' or 'uiconfirm' instead. |
| [`DQUESTDLG`](rules/compatibility.md#dquestdlg) | 'questdlg' is not recommended. Use 'uiconfirm' instead. |
| [`DSERIAL`](rules/compatibility.md#dserial) | 'serial' is not recommended. Use 'serialport' instead. |
| [`DGPIB2`](rules/compatibility.md#dgpib2) | 'gpib' is not recommended. Use 'visadev' instead. |
| [`DVISA2`](rules/compatibility.md#dvisa2) | 'visa' is not recommended. Use 'visadev' instead. |
| [`DTCPIP2`](rules/compatibility.md#dtcpip2) | 'tcpip' is not recommended. Use 'tcpclient' or 'tcpserver' instead. |
| [`DUDP2`](rules/compatibility.md#dudp2) | 'udp' is not recommended. Use 'udpport' instead. |
| [`DFMINUNC_OPT`](rules/compatibility.md#dfminuncopt) | 'optimset' is not recommended for newer solvers. Use 'optimoptions' instead. |
| [`DADDPREF`](rules/compatibility.md#daddpref) | 'addpref' is not recommended. Use 'matlab.settings' or 'settings' instead. |
| [`DGETPREF`](rules/compatibility.md#dgetpref) | 'getpref' is not recommended. Use 'matlab.settings' or 'settings' instead. |
| [`DSETPREF`](rules/compatibility.md#dsetpref) | 'setpref' is not recommended. Use 'matlab.settings' or 'settings' instead. |
| [`DRMPREF`](rules/compatibility.md#drmpref) | 'rmpref' is not recommended. Use 'matlab.settings' or 'settings' instead. |
| [`DISPREF`](rules/compatibility.md#dispref) | 'ispref' is not recommended. Use 'matlab.settings' or 'settings' instead. |
| [`DINLINE2`](rules/compatibility.md#dinline2) | 'inline' has been removed. Use anonymous functions (@(x) ...) instead. |
| [`DSYM2POLY`](rules/compatibility.md#dsym2poly) | 'sym2poly' is not recommended. Use 'coeffs' instead. |
| [`DFLIPUD`](rules/compatibility.md#dflipud) | For vectors, 'flipud' can be replaced by 'flip'. |
| [`DFLIPLR`](rules/compatibility.md#dfliplr) | For vectors, 'fliplr' can be replaced by 'flip'. |
| [`DISDIR`](rules/compatibility.md#disdir) | 'isdir' is not recommended. Use 'isfolder' instead. |
| [`DFILEATTRIB`](rules/compatibility.md#dfileattrib) | 'fileattrib' is not recommended. Use 'dir' with file attributes instead. |
| [`DVERLESSTHAN`](rules/compatibility.md#dverlessthan) | 'verLessThan' is not recommended. Use 'isMATLABReleaseOlderThan' instead. |
| [`DNARGCHK`](rules/compatibility.md#dnargchk) | 'nargchk' has been removed. Use 'narginchk' instead. |
| [`DNARGOUTCHK`](rules/compatibility.md#dnargoutchk) | The error-string form of 'nargoutchk' is not recommended. Use the narginchk-style calling form instead. |
| [`DCOMBNK`](rules/compatibility.md#dcombnk) | 'combnk' is not recommended. Use 'nchoosek' or 'combinations' instead. |
| [`DMATLABPOOL`](rules/compatibility.md#dmatlabpool) | 'matlabpool' has been removed. Use 'parpool' instead. |
| [`DDISTCOMP`](rules/compatibility.md#ddistcomp) | Older 'distributed' syntax is not recommended. Use newer parallel computing patterns. |
| [`DLABINDEX`](rules/compatibility.md#dlabindex) | 'labindex' is not recommended. Use 'spmdIndex' instead. |
| [`DNUMLABS`](rules/compatibility.md#dnumlabs) | 'numlabs' is not recommended. Use 'spmdSize' instead. |
| [`DLABBARRIER`](rules/compatibility.md#dlabbarrier) | 'labBarrier' is not recommended. Use 'spmdBarrier' instead. |
| [`DLABSEND`](rules/compatibility.md#dlabsend) | 'labSend' is not recommended. Use 'spmdSend' instead. |
| [`DLABRECEIVE`](rules/compatibility.md#dlabreceive) | 'labReceive' is not recommended. Use 'spmdReceive' instead. |
| [`DLABBROADCAST`](rules/compatibility.md#dlabbroadcast) | 'labBroadcast' is not recommended. Use 'spmdBroadcast' instead. |
| [`DLABSENDRECEIVE`](rules/compatibility.md#dlabsendreceive) | 'labSendReceive' is not recommended. Use 'spmdSendReceive' instead. |
| [`DLABPROBE`](rules/compatibility.md#dlabprobe) | 'labProbe' is not recommended. Use 'spmdProbe' instead. |
| [`DBARTLETT`](rules/compatibility.md#dbartlett) | 'bartlett' will be removed. Use 'barthannwin' or window functions from Signal Processing Toolbox. |
| [`DBLACKMANHARRIS`](rules/compatibility.md#dblackmanharris) | 'blackmanharris' is not recommended. Use 'blackmanharris' from Signal Processing Toolbox with updated syntax. |
| [`DBOHMANWIN`](rules/compatibility.md#dbohmanwin) | 'bohmanwin' window function syntax has been updated. |
| [`DGAUSSWIN`](rules/compatibility.md#dgausswin) | 'gausswin' window function syntax has been updated. |
| [`DKAISER`](rules/compatibility.md#dkaiser) | Use updated 'kaiser' window syntax. |
| [`DHAMMING`](rules/compatibility.md#dhamming) | Use updated 'hamming' window syntax. |
| [`DHANNING`](rules/compatibility.md#dhanning) | 'hanning' is not recommended. Use 'hann' instead. |
| [`DPBURG`](rules/compatibility.md#dpburg) | 'pburg' function syntax has been updated. |
| [`DPCOV`](rules/compatibility.md#dpcov) | 'pcov' function syntax has been updated. |
| [`DPMTM`](rules/compatibility.md#dpmtm) | 'pmtm' function syntax has been updated. |
| [`DPMUSIC`](rules/compatibility.md#dpmusic) | 'pmusic' function syntax has been updated. |
| [`DPWELCH`](rules/compatibility.md#dpwelch) | 'pwelch' function syntax has been updated. |
| [`DPYULEAR`](rules/compatibility.md#dpyulear) | 'pyulear' function syntax has been updated. |
| [`AXSTATE`](rules/compatibility.md#axstate) | 'axis('state')' has been removed. With appropriate code changes, use 'XLimMode', 'YLimMode', 'ZLimMode', 'Visible', 'XDir', and 'YDir' properties of an axes object instead. |
| [`FDDECI1`](rules/compatibility.md#fddeci1) | The 'Raised Cosine' response method of 'fdesign.decimator' object has been removed. With appropriate code changes use 'comm.RaisedCosineReceiveFilter' object instead. |
| [`FDDECI2`](rules/compatibility.md#fddeci2) | The 'Square Root Raised Cosine' response method of 'fdesign.decimator' object has been removed. With appropriate code changes use 'comm.RaisedCosineReceiveFilter' object instead. |
| [`FDINPO1`](rules/compatibility.md#fdinpo1) | The 'Raised Cosine' response method of 'fdesign.interpolator' object has been removed. With appropriate code changes use 'comm.RaisedCosineTransmitFilter' object instead. |
| [`FDINPO2`](rules/compatibility.md#fdinpo2) | The 'Square Root Raised Cosine' response method of 'fdesign.interpolator' object has been removed. With appropriate code changes use 'comm.RaisedCosineTransmitFilter' object instead. |
| [`WAVMENU`](rules/compatibility.md#wavmenu) | 'wavemenu' has been removed. With appropriate code changes, use Signal Multiresolution Analyzer, Wavelet Image Analyzer, Wavelet Signal Analyzer, Wavelet Signal Denoiser, or Wavelet Time-Frequency Analyzer instead. |
| [`WAVLAZ`](rules/compatibility.md#wavlaz) | 'waveletAnalyzer' has been removed. With appropriate code changes, use Signal Multiresolution Analyzer, Wavelet Image Analyzer, Wavelet Signal Analyzer, Wavelet Signal Denoiser, or Wavelet Time-Frequency Analyzer instead. |
| [`PRJET`](rules/compatibility.md#prjet) | 'project' has been removed. With appropriate code changes, use 'projfwd' instead. |
| [`RETPRI`](rules/compatibility.md#retpri) | Positional syntax for optional arguments has been removed from 'ret2price'. Use name-value pairs instead. |
| [`PRIRET`](rules/compatibility.md#priret) | Positional syntax for optional arguments has been removed from 'price2ret'. Use name-value pairs instead. |
| [`ACORR`](rules/compatibility.md#acorr) | Positional syntax for optional arguments has been removed from 'autocorr'. Use name-value pairs instead. |
| [`PCORR`](rules/compatibility.md#pcorr) | Positional syntax for optional arguments has been removed from 'parcorr'. Use name-value pairs instead. |
| [`XCORR`](rules/compatibility.md#xcorr) | Positional syntax for optional arguments has been removed from 'crosscorr'. Use name-value pairs instead. |
| [`HPFLTR`](rules/compatibility.md#hpfltr) | Positional syntax for optional arguments has been removed from 'hpfilter'. Use name-value pairs instead. |
| [`H5PGET`](rules/compatibility.md#h5pget) | 'H5P.get_dxpl_multi' has been removed. There is no simple replacement for this. |
| [`H5PSET`](rules/compatibility.md#h5pset) | 'H5P.set_dxpl_multi' has been removed. There is no simple replacement for this. |
| [`DGETST`](rules/compatibility.md#dgetst) | 'getstatus' will be removed in a future release. There is no simple replacement for this. |
| [`DMENUL`](rules/compatibility.md#dmenul) | 'menulabel' will be removed in a future release. There is no simple replacement for this. |
| [`DUIGET`](rules/compatibility.md#duiget) | 'uigettoolbar' will be removed in a future release. There is no simple replacement for this. |
| [`AVRREM`](rules/compatibility.md#avrrem) | The 'Aero.VirtualRealityAnimation' class no longer creates visualizations and will be removed in a future release. With appropriate code changes, use sim3d classes to create and view a 3D environment instead. |
| [`DAVIINF`](rules/compatibility.md#daviinf) | 'aviinfo' will be removed in a future release. With appropriate code changes, use 'VideoReader' instead. |
| [`DAFINF`](rules/compatibility.md#dafinf) | 'avifinfo' will be removed in a future release. With appropriate code changes, use 'VideoReader' instead. |
| [`COMMCCDF`](rules/compatibility.md#commccdf) | 'comm.CCDF' has been removed. With appropriate code changes, use 'powermeter' instead. |
| [`COMMIB`](rules/compatibility.md#commib) | 'comm.IntegerToBit' has been removed. With appropriate code changes, use 'int2bit' instead. |
| [`COMMSCOPESP`](rules/compatibility.md#commscopesp) | 'commscope.ScatterPlot' has been removed. With appropriate code changes, use 'comm.ConstellationDiagram' instead. |
| [`COMMBSC`](rules/compatibility.md#commbsc) | 'comm.BinarySymmetricChannel' has been removed. With appropriate code changes, use 'BSC' instead. |
| [`RLCHN`](rules/compatibility.md#rlchn) | 'rayleighchan' has been removed. With appropriate code changes, use 'comm.RayleighChannel' instead. |
| [`RICHN`](rules/compatibility.md#richn) | 'ricianchan' has been removed. With appropriate code changes, use 'comm.RicianChannel' instead. |
| [`LEGCHN`](rules/compatibility.md#legchn) | 'legacychannelsim' has been removed. There is no simple replacement for this. |
| [`DOPJKS`](rules/compatibility.md#dopjks) | 'doppler.jakes' has been removed. With appropriate code changes, use 'doppler' instead. |
| [`DOPRJKS`](rules/compatibility.md#doprjks) | 'doppler.rjakes' has been removed. With appropriate code changes, use 'doppler' instead. |
| [`DOPAJKS`](rules/compatibility.md#dopajks) | 'doppler.ajakes' has been removed. With appropriate code changes, use 'doppler' instead. |
| [`DOPFLT`](rules/compatibility.md#dopflt) | 'doppler.flat' has been removed. With appropriate code changes, use 'doppler' instead. |
| [`DOPBLL`](rules/compatibility.md#dopbll) | 'doppler.bell' has been removed. With appropriate code changes, use 'doppler' instead. |
| [`DOPRNDD`](rules/compatibility.md#doprndd) | 'doppler.rounded' has been removed. With appropriate code changes, use 'doppler' instead. |
| [`DOPGSS`](rules/compatibility.md#dopgss) | 'doppler.gaussian' has been removed. With appropriate code changes, use 'doppler' instead. |
| [`DOPBGSS`](rules/compatibility.md#dopbgss) | 'doppler.bigaussian' has been removed. With appropriate code changes, use 'doppler' instead. |
| [`COMMPSKC`](rules/compatibility.md#commpskc) | 'comm.PSKCoarseFrequencyEstimator' has been removed. With appropriate code changes, use 'comm.CoarseFrequencyCompensator' instead. |
| [`COMMQAMC`](rules/compatibility.md#commqamc) | 'comm.QAMCoarseFrequencyEstimator' has been removed. With appropriate code changes, use 'comm.CoarseFrequencyCompensator' instead. |
| [`DEVM`](rules/compatibility.md#devm) | 'commmeasure.EVM' has been removed. With appropriate code changes, use 'comm.EVM' instead. |
| [`DMER`](rules/compatibility.md#dmer) | 'commmeasure.MER' has been removed. With appropriate code changes, use 'comm.MER' instead. |
| [`DACPR`](rules/compatibility.md#dacpr) | 'commmeasure.ACPR' has been removed. With appropriate code changes, use 'comm.ACPR' instead. |
| [`CRCGE`](rules/compatibility.md#crcge) | 'crc.generator' has been removed. With appropriate code changes, use 'comm.CRCGenerator' instead. |
| [`CRCDE`](rules/compatibility.md#crcde) | 'crc.detector' has been removed. With appropriate code changes, use 'comm.CRCDetector' instead. |
| [`COMMCRCGEN`](rules/compatibility.md#commcrcgen) | 'comm.CRCGenerator' will be removed in a future release. With appropriate code changes, use 'crcGenerate' instead. |
| [`COMMCRCDET`](rules/compatibility.md#commcrcdet) | 'comm.CRCDetector' will be removed in a future release. With appropriate code changes, use 'crcDetect' instead. |
| [`COMMDVBS2LDPC`](rules/compatibility.md#commdvbs2ldpc) | 'dvbs2ldpc' will be removed in a future release. With appropriate code changes, use 'dvbsLDPCPCM' instead. |
| [`CMDFE`](rules/compatibility.md#cmdfe) | 'dfe' has been removed. With appropriate code changes, use 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMDFEEQ`](rules/compatibility.md#cmdfeeq) | 'equalizer.dfe' has been removed. With appropriate code changes, use 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMLRQ`](rules/compatibility.md#cmlrq) | 'lineareq' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' instead. |
| [`CMLRQEQ`](rules/compatibility.md#cmlrqeq) | 'equalizer.lineareq' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' instead. |
| [`CMLMSAD`](rules/compatibility.md#cmlmsad) | 'adaptalg.lms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMRLSAD`](rules/compatibility.md#cmrlsad) | 'adaptalg.rls' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMCMAAD`](rules/compatibility.md#cmcmaad) | 'adaptalg.cma' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMSLMSAD`](rules/compatibility.md#cmslmsad) | 'adaptalg.signlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMVLMSAD`](rules/compatibility.md#cmvlmsad) | 'adaptalg.varlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMNLMSAD`](rules/compatibility.md#cmnlmsad) | 'adaptalg.normlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMLMS`](rules/compatibility.md#cmlms) | 'lms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMRLS`](rules/compatibility.md#cmrls) | 'rls' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMCMA`](rules/compatibility.md#cmcma) | 'cma' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMSLMS`](rules/compatibility.md#cmslms) | 'signlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMVLMS`](rules/compatibility.md#cmvlms) | 'varlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMNLMS`](rules/compatibility.md#cmnlms) | 'normlms' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`CMEQU`](rules/compatibility.md#cmequ) | 'equalize' has been removed. With appropriate code changes, use 'comm.LinearEqualizer' or 'comm.DecisionFeedbackEqualizer' instead. |
| [`DBHENC`](rules/compatibility.md#dbhenc) | 'fec.bchenc' has been removed. With appropriate code changes, use 'comm.BCHEncoder' instead. |
| [`DBCHDEC`](rules/compatibility.md#dbchdec) | 'fec.bchdec' has been removed. With appropriate code changes, use 'comm.BCHDecoder' instead. |
| [`DRSENC`](rules/compatibility.md#drsenc) | 'fec.rsenc' has been removed. With appropriate code changes, use 'comm.RSEncoder' instead. |
| [`DRSDEC`](rules/compatibility.md#drsdec) | 'fec.rsdec' has been removed. With appropriate code changes, use 'comm.RSDecoder' instead. |
| [`COMMGPUAWGN`](rules/compatibility.md#commgpuawgn) | 'comm.gpu.AWGNChannel' will be removed in a future release. With appropriate code changes, use 'awgn' instead. |
| [`COMMGPULDPCDEC`](rules/compatibility.md#commgpuldpcdec) | 'comm.gpu.LDPCDecoder' will be removed in a future release. With appropriate code changes, use 'ldpcDecode' instead. |
| [`COMMGPUBLKINTRLV`](rules/compatibility.md#commgpublkintrlv) | 'comm.gpu.BlockInterleaver' will be removed in a future release. With appropriate code changes, use 'intrlv' instead. |
| [`COMMGPUBLKDEINTRLV`](rules/compatibility.md#commgpublkdeintrlv) | 'comm.gpu.BlockDeinterleaver' will be removed in a future release. With appropriate code changes, use 'deintrlv' instead. |
| [`COMMGPUCONVINTRLV`](rules/compatibility.md#commgpuconvintrlv) | 'comm.gpu.ConvolutionalInterleaver' will be removed in a future release. With appropriate code changes, use 'convintrlv' instead. |
| [`COMMGPUCONVDEINTRLV`](rules/compatibility.md#commgpuconvdeintrlv) | 'comm.gpu.ConvolutionalDeinterleaver' will be removed in a future release. With appropriate code changes, use 'convdeintrlv' instead. |
| [`COMMGPUPSKMODULATOR`](rules/compatibility.md#commgpupskmodulator) | 'comm.gpu.PSKModulator' will be removed in a future release. With appropriate code changes, use 'pskmod' instead. |
| [`COMMGPUPSKDEMODULATOR`](rules/compatibility.md#commgpupskdemodulator) | 'comm.gpu.PSKDemodulator' will be removed in a future release. With appropriate code changes, use 'pskdemod' instead. |
| [`LDPCE`](rules/compatibility.md#ldpce) | 'comm.LDPCEncoder' has been removed. With appropriate code changes, use 'ldpcEncode' instead. |
| [`LDPCD`](rules/compatibility.md#ldpcd) | 'comm.LDPCDecoder' has been removed. With appropriate code changes, use 'ldpcDecode' instead. |
| [`COMMLMC`](rules/compatibility.md#commlmc) | 'comm.LTEMIMOChannel' has been removed. With appropriate code changes, use 'comm.MIMOChannel' instead. |
| [`QMOD`](rules/compatibility.md#qmod) | 'modem.qammod' has been removed. With appropriate code changes, use 'qammod' instead. |
| [`QDEMOD`](rules/compatibility.md#qdemod) | 'modem.qamdemod' has been removed. With appropriate code changes, use 'qamdemod' instead. |
| [`DMOD`](rules/compatibility.md#dmod) | 'modem.dpskmod' has been removed. With appropriate code changes, use 'comm.DPSKModulator' instead. |
| [`DDEMOD`](rules/compatibility.md#ddemod) | 'modem.dpskdemod' has been removed. With appropriate code changes, use 'comm.DPSKDemodulator' instead. |
| [`OMOD`](rules/compatibility.md#omod) | 'modem.oqpskmod' has been removed. With appropriate code changes, use 'comm.OQPSKModulator' instead. |
| [`ODEMOD`](rules/compatibility.md#odemod) | 'modem.oqpskdemod' has been removed. With appropriate code changes, use 'comm.OQPSKDemodulator' instead. |
| [`PAMOD`](rules/compatibility.md#pamod) | 'modem.pammod' has been removed. With appropriate code changes, use 'pammod' instead. |
| [`PADEMOD`](rules/compatibility.md#pademod) | 'modem.pamdemod' has been removed. With appropriate code changes, use 'pamdemod' instead. |
| [`MMOD`](rules/compatibility.md#mmod) | 'modem.mskmod' has been removed. With appropriate code changes, use 'comm.MSKModulator' or 'mskmod' instead. |
| [`MDEMOD`](rules/compatibility.md#mdemod) | 'modem.mskdemod' has been removed. With appropriate code changes, use 'comm.MSKDemodulator' or 'mskdemod' instead. |
| [`GMOD`](rules/compatibility.md#gmod) | 'modem.genqammod' has been removed. With appropriate code changes, use 'genqammod' or 'comm.GeneralQAMModulator' instead. |
| [`GDEMOD`](rules/compatibility.md#gdemod) | 'modem.genqamdemod' has been removed. With appropriate code changes, use 'genqamdemod' or 'comm.GeneralQAMDemodulator' instead. |
| [`PSMOD`](rules/compatibility.md#psmod) | 'modem.pskmod' has been removed. With appropriate code changes, use 'pskmod' instead. |
| [`PSDEMOD`](rules/compatibility.md#psdemod) | 'modem.pskdemod' has been removed. With appropriate code changes, use 'pskdemod' instead. |
| [`OQPMOD`](rules/compatibility.md#oqpmod) | 'oqpskmod' has been removed. With appropriate code changes, use 'comm.OQPSKModulator' instead. |
| [`OQPDEM`](rules/compatibility.md#oqpdem) | 'oqpskdemod' has been removed. With appropriate code changes, use 'comm.OQPSKDemodulator' instead. |
| [`COMMPSKMSO`](rules/compatibility.md#commpskmso) | 'comm.PSKModulator' will be removed in a future release. With appropriate code changes, use 'pskmod' instead. |
| [`COMMPSKDSO`](rules/compatibility.md#commpskdso) | 'comm.PSKDemodulator' will be removed in a future release. With appropriate code changes, use 'pskdemod' instead. |
| [`COMMBPSKMSO`](rules/compatibility.md#commbpskmso) | 'comm.BPSKModulator' will be removed in a future release. With appropriate code changes, use 'pskmod' instead. |
| [`COMMBPSKDSO`](rules/compatibility.md#commbpskdso) | 'comm.BPSKDemodulator' will be removed in a future release. With appropriate code changes, use 'pskdemod' instead. |
| [`COMMQPSKMSO`](rules/compatibility.md#commqpskmso) | 'comm.QPSKModulator' will be removed in a future release. With appropriate code changes, use 'pskmod' instead. |
| [`COMMQPSKDSO`](rules/compatibility.md#commqpskdso) | 'comm.QPSKDemodulator' will be removed in a future release. With appropriate code changes, use 'pskdemod' instead. |
| [`CMPSKCPS`](rules/compatibility.md#cmpskcps) | 'comm.PSKCarrierPhaseSynchronizer' has been removed. With appropriate code changes, use 'comm.CarrierSynchronizer' instead. |
| [`COMMQAMM`](rules/compatibility.md#commqamm) | 'comm.RectangularQAMModulator' has been removed. With appropriate code changes, use 'qammod' instead. |
| [`COMMQAMD`](rules/compatibility.md#commqamd) | 'comm.RectangularQAMDemodulator' has been removed. With appropriate code changes, use 'qamdemod' instead. |
| [`CMELGTS`](rules/compatibility.md#cmelgts) | 'comm.EarlyLateGateTimingSynchronizer' has been removed. With appropriate code changes, use 'comm.SymbolSynchronizer' instead. |
| [`CMGTS`](rules/compatibility.md#cmgts) | 'comm.GardnerTimingSynchronizer' has been removed. With appropriate code changes, use 'comm.SymbolSynchronizer' instead. |
| [`CMMMTS`](rules/compatibility.md#cmmmts) | 'comm.MuellerMullerTimingSynchronizer' has been removed. With appropriate code changes, use 'comm.SymbolSynchronizer' instead. |
| [`ALDEINT`](rules/compatibility.md#aldeint) | 'comm.AlgebraicDeinterleaver' has been removed. With appropriate code changes, use 'algdeintrlv' instead. |
| [`ALINT`](rules/compatibility.md#alint) | 'comm.AlgebraicInterleaver' has been removed. With appropriate code changes, use 'algintrlv' instead. |
| [`BLKDEINT`](rules/compatibility.md#blkdeint) | 'comm.BlockDeinterleaver' has been removed. With appropriate code changes, use 'deintrlv' instead. |
| [`BLKINT`](rules/compatibility.md#blkint) | 'comm.BlockInterleaver' has been removed. With appropriate code changes, use 'intrlv' instead. |
| [`MATDEINT`](rules/compatibility.md#matdeint) | 'comm.MatrixDeinterleaver' has been removed. With appropriate code changes, use 'matdeintrlv' instead. |
| [`MATINT`](rules/compatibility.md#matint) | 'comm.MatrixInterleaver' has been removed. With appropriate code changes, use 'matintrlv' instead. |
| [`MATHSDEINT`](rules/compatibility.md#mathsdeint) | 'comm.MatrixHelicalScanDeinterleaver' has been removed. With appropriate code changes, use 'helscandeintrlv' instead. |
| [`MATHSINT`](rules/compatibility.md#mathsint) | 'comm.MatrixHelicalScanInterleaver' has been removed. With appropriate code changes, use 'helscanintrlv' instead. |
| [`ZADOF`](rules/compatibility.md#zadof) | 'lteZadoffChuSeq' has been removed. Use 'zadoffChuSeq' instead, which is a direct replacement. |
| [`CMCPMCPS`](rules/compatibility.md#cmcpmcps) | 'comm.CPMCarrierPhaseSynchronizer' has been removed. With appropriate code changes, use 'comm.CarrierSynchronizer' instead. |
| [`SESSIONR`](rules/compatibility.md#sessionr) | 'daq.reset' will be removed in a future release. Use 'daqreset' instead, which is a direct replacement. |
| [`SESSIONGD`](rules/compatibility.md#sessiongd) | 'daq.getDevices' will be removed in a future release. With appropriate code changes, use 'daqlist' instead. |
| [`SESSIONGV`](rules/compatibility.md#sessiongv) | 'daq.getVendors' will be removed in a future release. Use 'daqvendorlist' instead, which is a direct replacement. |
| [`AFBLMSFFT`](rules/compatibility.md#afblmsfft) | 'adaptfilt.blmsfft' has been removed. There is no simple replacement for this. |
| [`AFADJLMS`](rules/compatibility.md#afadjlms) | 'adaptfilt.adjlms' has been removed. There is no simple replacement for this. |
| [`AFDLMS`](rules/compatibility.md#afdlms) | 'adaptfilt.dlms' has been removed. There is no simple replacement for this. |
| [`AFPBFDAF`](rules/compatibility.md#afpbfdaf) | 'adaptfilt.pbfdaf' has been removed. There is no simple replacement for this. |
| [`AFPBUFDAF`](rules/compatibility.md#afpbufdaf) | 'adaptfilt.pbufdaf' has been removed. There is no simple replacement for this. |
| [`AFTDAFDCT`](rules/compatibility.md#aftdafdct) | 'adaptfilt.tdafdct' has been removed. There is no simple replacement for this. |
| [`AFTFAFDFT`](rules/compatibility.md#aftfafdft) | 'adaptfilt.tfafdft' has been removed. There is no simple replacement for this. |
| [`AFSE`](rules/compatibility.md#afse) | 'adaptfilt.se' has been removed. With appropriate code changes, use 'dsp.LMSFilter' instead. |
| [`AFSD`](rules/compatibility.md#afsd) | 'adaptfilt.sd' has been removed. With appropriate code changes, use 'dsp.LMSFilter' instead. |
| [`AFSS`](rules/compatibility.md#afss) | 'adaptfilt.ss' has been removed. With appropriate code changes, use 'dsp.LMSFilter' instead. |
| [`AFQRDRLS`](rules/compatibility.md#afqrdrls) | 'adaptfilt.qrdrls' has been removed. With appropriate code changes, use 'dsp.RLSFilter' instead. |
| [`AFSWRLS`](rules/compatibility.md#afswrls) | 'adaptfilt.swrls' has been removed. With appropriate code changes, use 'dsp.RLSFilter' instead. |
| [`AFHRLS`](rules/compatibility.md#afhrls) | 'adaptfilt.hrls' has been removed. With appropriate code changes, use 'dsp.RLSFilter' instead. |
| [`AFHSWRLS`](rules/compatibility.md#afhswrls) | 'adaptfilt.hswrls' has been removed. With appropriate code changes, use 'dsp.RLSFilter' instead. |
| [`AFSWFTF`](rules/compatibility.md#afswftf) | 'adaptfilt.swftf' has been removed. With appropriate code changes, use 'dsp.FastTransversalFilter' instead. |
| [`AFFTF`](rules/compatibility.md#afftf) | 'adaptfilt.ftf' has been removed. With appropriate code changes, use 'dsp.FastTransversalFilter' instead. |
| [`AFAP`](rules/compatibility.md#afap) | 'adaptfilt.ap' has been removed. With appropriate code changes, use 'dsp.AffineProjectionFilter' instead. |
| [`AFAPRU`](rules/compatibility.md#afapru) | 'adaptfilt.apru' has been removed. With appropriate code changes, use 'dsp.AffineProjectionFilter' instead. |
| [`AFBAP`](rules/compatibility.md#afbap) | 'adaptfilt.bap' has been removed. With appropriate code changes, use 'dsp.AffineProjectionFilter' instead. |
| [`AFGAL`](rules/compatibility.md#afgal) | 'adaptfilt.gal' has been removed. With appropriate code changes, use 'dsp.AdaptiveLatticeFilter' instead. |
| [`AFLSL`](rules/compatibility.md#aflsl) | 'adaptfilt.lsl' has been removed. With appropriate code changes, use 'dsp.AdaptiveLatticeFilter' instead. |
| [`AFQRDLSL`](rules/compatibility.md#afqrdlsl) | 'adaptfilt.qrdlsl' has been removed. With appropriate code changes, use 'dsp.AdaptiveLatticeFilter' instead. |
| [`AFFILTXLMS`](rules/compatibility.md#affiltxlms) | 'adaptfilt.filtxlms' has been removed. With appropriate code changes, use 'dsp.FilteredXLMSFilter' instead. |
| [`AFFDAF`](rules/compatibility.md#affdaf) | 'adaptfilt.fdaf' has been removed. With appropriate code changes, use 'dsp.FrequencyDomainAdaptiveFilter' instead. |
| [`AFUFDAF`](rules/compatibility.md#afufdaf) | 'adaptfilt.ufdaf' has been removed. With appropriate code changes, use 'dsp.FrequencyDomainAdaptiveFilter' instead. |
| [`DSPLTS`](rules/compatibility.md#dsplts) | 'dsp.LowerTriangularSolver' has been removed. With appropriate code changes, use 'mldivide' function or '\\' operator instead. |
| [`DSPUTS`](rules/compatibility.md#dsputs) | 'dsp.UpperTriangularSolver' has been removed. With appropriate code changes, use 'mldivide' function or '\\' operator instead. |
| [`DSPPMS`](rules/compatibility.md#dsppms) | 'dsp.PulseMetrics' has been removed. With appropriate code changes, use 'dutycycle', 'midcross', 'pulseperiod', 'pulsesep' or 'pulsewidth' instead. |
| [`DSPTMS`](rules/compatibility.md#dsptms) | 'dsp.TransitionMetrics' has been removed. With appropriate code changes, use 'falltime', 'overshoot', 'risetime', 'settlingtime', 'slewrate' or 'undershoot' instead. |
| [`FDESPULSESH`](rules/compatibility.md#fdespulsesh) | 'fdesign.pulseshaping' has been removed. With appropriate code changes, use 'rcosdesign' or 'gaussdesign' instead. |
| [`MCDECIM`](rules/compatibility.md#mcdecim) | 'mfilt.cicdecim' will be removed in a future release. With appropriate code changes, use 'dsp.CICDecimator' instead. |
| [`MCINTERP`](rules/compatibility.md#mcinterp) | 'mfilt.cicinterp' has been removed. With appropriate code changes, use 'dsp.CICInterpolator' instead. |
| [`MFARROW`](rules/compatibility.md#mfarrow) | 'mfilt.farrowsrc' has been removed. With appropriate code changes, use 'dsp.FarrowRateConverter' instead. |
| [`MFDECIM`](rules/compatibility.md#mfdecim) | 'mfilt.firdecim' will be removed in a future release. With appropriate code changes, use 'dsp.FIRDecimator' instead. |
| [`MFTDECIM`](rules/compatibility.md#mftdecim) | 'mfilt.firtdecim' will be removed in a future release. With appropriate code changes, use 'dsp.FIRDecimator' instead. |
| [`MFINTERP`](rules/compatibility.md#mfinterp) | 'mfilt.firinterp' has been removed. With appropriate code changes, use 'dsp.FIRInterpolator' instead. |
| [`MFSRC`](rules/compatibility.md#mfsrc) | 'mfilt.firsrc' will be removed in a future release. With appropriate code changes, use 'dsp.FIRRateConverter' instead. |
| [`MFFTFINTERP`](rules/compatibility.md#mfftfinterp) | 'mfilt.fftfirinterp' has been removed. With appropriate code changes, use 'dsp.FIRInterpolator' instead. |
| [`MHINTERP`](rules/compatibility.md#mhinterp) | 'mfilt.holdinterp' has been removed. With appropriate code changes, use 'dsp.CICInterpolator' instead. |
| [`MIDECIM`](rules/compatibility.md#midecim) | 'mfilt.iirdecim' will be removed in a future release. With appropriate code changes, use 'dsp.IIRHalfbandDecimator' instead. |
| [`MIINTERP`](rules/compatibility.md#miinterp) | 'mfilt.iirinterp' has been removed. With appropriate code changes, use 'dsp.IIRHalfbandInterpolator' instead. |
| [`MIWDFDECIM`](rules/compatibility.md#miwdfdecim) | 'mfilt.iirwdfdecim' will be removed in a future release. With appropriate code changes, use 'dsp.IIRHalfbandDecimator' instead. |
| [`MIWDFINTERP`](rules/compatibility.md#miwdfinterp) | 'mfilt.iirwdfinterp' will be removed in a future release. With appropriate code changes, use 'dsp.IIRHalfbandInterpolator' instead. |
| [`MLINTERP`](rules/compatibility.md#mlinterp) | 'mfilt.linearinterp' has been removed. With appropriate code changes, use 'dsp.CICInterpolator' instead. |
| [`MFFDCM`](rules/compatibility.md#mffdcm) | 'mfilt.firfracdecim' has been removed. With appropriate code changes, use 'dsp.FIRRateConverter' instead. |
| [`MFFINTRP`](rules/compatibility.md#mffintrp) | 'mfilt.firfracinterp' has been removed. With appropriate code changes, use 'dsp.FIRRateConverter' instead. |
| [`DSPTS`](rules/compatibility.md#dspts) | 'dsp.TimeScope' will be removed in a future release. Use 'timescope' instead, which is a direct replacement. |
| [`DSPCEP2LPC`](rules/compatibility.md#dspcep2lpc) | 'dsp.CepstralToLPC' has been removed. There is no simple replacement for this. |
| [`DSPLPC2AUTOCORR`](rules/compatibility.md#dsplpc2autocorr) | 'dsp.LPCToAutocorrelation' has been removed. With appropriate code changes, use 'poly2ac' instead. |
| [`DSPLPC2CEP`](rules/compatibility.md#dsplpc2cep) | 'dsp.LPCToCepstral' has been removed. There is no simple replacement for this. |
| [`DSPLPC2LSF`](rules/compatibility.md#dsplpc2lsf) | 'dsp.LPCToLSF' has been removed. With appropriate code changes, use 'poly2lsf' instead. |
| [`DSPLPC2RC`](rules/compatibility.md#dsplpc2rc) | 'dsp.LPCToRC' has been removed. With appropriate code changes, use 'poly2rc' instead. |
| [`DSPLSF2LPC`](rules/compatibility.md#dsplsf2lpc) | 'dsp.LSFToLPC' has been removed. With appropriate code changes, use 'lsf2poly' instead. |
| [`DSPLSP2LPC`](rules/compatibility.md#dsplsp2lpc) | 'dsp.LSPToLPC' has been removed. There is no simple replacement for this. |
| [`DSPRC2AUTOCORR`](rules/compatibility.md#dsprc2autocorr) | 'dsp.RCToAutocorrelation' has been removed. With appropriate code changes, use 'rc2ac' instead. |
| [`DSPRC2LPC`](rules/compatibility.md#dsprc2lpc) | 'dsp.RCToLPC' has been removed. With appropriate code changes, use 'rc2poly' instead. |
| [`DSPBURGEST`](rules/compatibility.md#dspburgest) | 'dsp.BurgAREstimator' has been removed. With appropriate code changes, use 'arburg' instead. |
| [`DSPBURGSPECEST`](rules/compatibility.md#dspburgspecest) | 'dsp.BurgSpectrumEstimator' has been removed. With appropriate code changes, use 'pburg' instead. |
| [`DSPDCT`](rules/compatibility.md#dspdct) | 'dsp.DCT' has been removed. With appropriate code changes, use 'dct' instead. |
| [`DSPIDCT`](rules/compatibility.md#dspidct) | 'dsp.IDCT' has been removed. With appropriate code changes, use 'idct' instead. |
| [`DSPPARAMEQ`](rules/compatibility.md#dspparameq) | 'dsp.ParametricEQFilter' has been removed. With appropriate code changes, use 'designParamEQ' or 'MultibandParametricEQ' instead. |
| [`DSPSCALARQUANTDEC`](rules/compatibility.md#dspscalarquantdec) | 'dsp.ScalarQuantizerDecoder' has been removed. There is no simple replacement for this. |
| [`DSPSCALARQUANTENC`](rules/compatibility.md#dspscalarquantenc) | 'dsp.ScalarQuantizerEncoder' has been removed. There is no simple replacement for this. |
| [`DSPUNIDEC`](rules/compatibility.md#dspunidec) | 'dsp.UniformDecoder' has been removed. With appropriate code changes, use 'udecode' instead. |
| [`DSPUNIENC`](rules/compatibility.md#dspunienc) | 'dsp.UniformEncoder' has been removed. With appropriate code changes, use 'uencode' instead. |
| [`DSPVECQUANTDEC`](rules/compatibility.md#dspvecquantdec) | 'dsp.VectorQuantizerDecoder' has been removed. There is no simple replacement for this. |
| [`DSPVECQUANTENC`](rules/compatibility.md#dspvecquantenc) | 'dsp.VectorQuantizerEncoder' has been removed. There is no simple replacement for this. |
| [`DSPSTATELVLS`](rules/compatibility.md#dspstatelvls) | 'dsp.StateLevels' has been removed. With appropriate code changes, use 'statelevels' instead. |
| [`FAFD`](rules/compatibility.md#fafd) | 'farrow.fd' has been removed. With appropriate code changes, use 'dfilt.farrowfd' instead. |
| [`FALFD`](rules/compatibility.md#falfd) | 'farrow.linearfd' has been removed. With appropriate code changes, use 'dfilt.farrowlinearfd' instead. |
| [`OPCDASUPP`](rules/compatibility.md#opcdasupp) | 'opc.daSupport' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| [`OPCOPNOSF`](rules/compatibility.md#opcopnosf) | 'openosf' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| [`OPCQID`](rules/compatibility.md#opcqid) | 'opcqid' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| [`OPCQSTR`](rules/compatibility.md#opcqstr) | 'opcqstr' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| [`OPCQPRT`](rules/compatibility.md#opcqprt) | 'opcqparts' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| [`OPCDAQS`](rules/compatibility.md#opcdaqs) | 'opc.daQualityString' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| [`OPCDAEXPL`](rules/compatibility.md#opcdaexpl) | 'opcDataAccessExplorer' has been removed along with the support for OPC DA. To access live OPC data, use the OPC UA standard instead. |
| [`I2CFUN`](rules/compatibility.md#i2cfun) | 'i2c' will be removed in a future release. With appropriate code changes, use 'device' method of 'ni845x' or 'aardvark' instead. |
| [`IVIDC`](rules/compatibility.md#ividc) | 'instrument.ivic.IviDCPwr' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| [`IVIDM`](rules/compatibility.md#ividm) | 'instrument.ivic.IviDmm' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| [`IVIFG`](rules/compatibility.md#ivifg) | 'instrument.ivic.IviFgen' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| [`IVIPW`](rules/compatibility.md#ivipw) | 'instrument.ivic.IviPwrMeter' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| [`IVIRF`](rules/compatibility.md#ivirf) | 'instrument.ivic.IviRFSigGen' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| [`IVISP`](rules/compatibility.md#ivisp) | 'instrument.ivic.IviSpecAn' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| [`IVISW`](rules/compatibility.md#ivisw) | 'instrument.ivic.IviSwtch' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| [`IVISC`](rules/compatibility.md#ivisc) | 'instrument.ivic.IviScope' will be removed in a future release. With appropriate code changes, use 'ividev' instead. |
| [`INSTRR`](rules/compatibility.md#instrr) | 'instrreset' will be removed in a future release. With appropriate code changes, instead use 'delete(serialportfind)', 'delete(tcpclientfind)', 'delete(tcpserverfind)', 'delete(udpportfind)', 'delete(visadevfind)', 'delete(aardvarkfind)', 'delete(ni845xfind)', or 'delete(icdevicefind)' when 'LegacyMode' for icdevice is false. |
| [`INSTRH`](rules/compatibility.md#instrh) | 'instrhelp' will be removed in a future release. Use 'help' instead, which is a direct replacement. |
| [`INSTRF`](rules/compatibility.md#instrf) | 'instrfind' will be removed in a future release. With appropriate code changes, instead use 'serialportfind', 'tcpclientfind', 'tcpserverfind', 'udpportfind', 'visadevfind', 'aardvarkfind', 'ni845xfind', or 'icdevicefind' when 'LegacyMode' for icdevice is false. |
| [`INSTFA`](rules/compatibility.md#instfa) | 'instrfindall' will be removed in a future release. With appropriate code changes, instead use 'serialportfind', 'tcpclientfind', 'tcpserverfind', 'udpportfind', 'visadevfind', 'aardvarkfind', 'ni845xfind', or 'icdevicefind' when 'LegacyMode' for icdevice is false. |
| [`INSTCA`](rules/compatibility.md#instca) | 'instrcallback' will be removed in a future release. There is no simple replacement for this. |
| [`INSTRN`](rules/compatibility.md#instrn) | 'instrnotify' will be removed in a future release. There is no simple replacement for this. |
| [`MKMID`](rules/compatibility.md#mkmid) | 'makemid' will be removed in a future release. There is no simple replacement for this. |
| [`MIEIT`](rules/compatibility.md#mieit) | 'midedit' will be removed in a future release. There is no simple replacement for this. |
| [`MIDTT`](rules/compatibility.md#midtt) | 'midtest' will be removed in a future release. There is no simple replacement for this. |
| [`TMTL`](rules/compatibility.md#tmtl) | 'tmtool' will be removed in a future release. Use 'serialExplorer', 'tcpipExplorer', 'udpExplorer', 'visaExplorer', or 'instrumentExplorer' instead. |
| [`IMJAVA`](rules/compatibility.md#imjava) | 'im2java' has been removed. There is no simple replacement for this. |
| [`ECF2LV`](rules/compatibility.md#ecf2lv) | 'ecef2lv' has been removed. With appropriate code changes, use 'ecef2enu' instead. |
| [`LV2ECF`](rules/compatibility.md#lv2ecf) | 'lv2ecef' has been removed. With appropriate code changes, use 'enu2ecef' instead. |
| [`RDFLDS`](rules/compatibility.md#rdflds) | 'readfields' has been removed. With appropriate code changes, use 'readmatrix', 'readtable', or a different file import function instead. |
| [`RDMTX`](rules/compatibility.md#rdmtx) | 'readmtx' has been removed. With appropriate code changes, use 'readmatrix', 'readtable', or a different file import function instead. |
| [`RDFK5`](rules/compatibility.md#rdfk5) | 'readfk5' has been removed. There is no simple replacement for this. |
| [`SPCRD`](rules/compatibility.md#spcrd) | 'spcread' has been removed. With appropriate code changes, use 'readmatrix' instead. |
| [`COLORM`](rules/compatibility.md#colorm) | 'colorm' has been removed. There is no simple replacement for this. |
| [`GTSEED`](rules/compatibility.md#gtseed) | 'getseeds' has been removed. There is no simple replacement for this. |
| [`MKMAP`](rules/compatibility.md#mkmap) | 'makemapped' has been removed. There is no simple replacement for this. |
| [`MOBJS`](rules/compatibility.md#mobjs) | 'mobjects' has been removed. There is no simple replacement for this. |
| [`SEEDM`](rules/compatibility.md#seedm) | 'seedm' has been removed. There is no simple replacement for this. |
| [`LKBLNK`](rules/compatibility.md#lkblnk) | 'leadblnk' has been removed. With appropriate code changes, use 'strtrim' instead. |
| [`SFTSPC`](rules/compatibility.md#sftspc) | 'shiftspc' has been removed. With appropriate code changes, use 'strjust' instead. |
| [`GTR2GLT`](rules/compatibility.md#gtr2glt) | 'geocentric2geodeticLat' has been removed. With appropriate code changes, use 'geodeticLatitudeFromGeocentric' instead. |
| [`GLT2GTR`](rules/compatibility.md#glt2gtr) | 'geodetic2geocentricLat' has been removed. With appropriate code changes, use 'geocentricLatitude' instead. |
| [`MEXTM`](rules/compatibility.md#mextm) | 'extractm' has been removed. With appropriate code changes, use geospatial tables instead. |
| [`QRYDT`](rules/compatibility.md#qrydt) | 'qrydata' has been removed. There is no simple replacement for this. |
| [`CMBNT`](rules/compatibility.md#cmbnt) | 'combntns' has been removed. Use 'nchoosek' instead, which is a direct replacement. |
| [`FPSNM`](rules/compatibility.md#fpsnm) | 'fipsname' has been removed. With appropriate code changes, use 'readgeotable' instead. |
| [`GREPF`](rules/compatibility.md#grepf) | 'grepfields' has been removed. With appropriate code changes, use 'textscan' instead. |
| [`TRGLN`](rules/compatibility.md#trgln) | 'tgrline' has been removed. With appropriate code changes, use 'readgeotable' instead. |
| [`CLRUI`](rules/compatibility.md#clrui) | 'colorui' has been removed. With appropriate code changes, use 'uisetcolor' instead. |
| [`COMET`](rules/compatibility.md#comet) | 'cometm' has been removed. With appropriate code changes, use 'comet' instead. |
| [`COMET3`](rules/compatibility.md#comet3) | 'comet3m' has been removed. With appropriate code changes, use 'comet3' instead. |
| [`MLYER`](rules/compatibility.md#mlyer) | 'mlayers' has been removed. There is no simple replacement for this. |
| [`RSTCK`](rules/compatibility.md#rstck) | 'restack' has been removed. With appropriate code changes, use 'uistack' instead. |
| [`RTLYR`](rules/compatibility.md#rtlyr) | 'rootlayr' has been removed. There is no simple replacement for this. |
| [`EASTF`](rules/compatibility.md#eastf) | 'eastof' has been removed. With appropriate code changes, use 'mod' instead. |
| [`WESTF`](rules/compatibility.md#westf) | 'westof' has been removed. With appropriate code changes, use 'mod' instead. |
| [`AT2GD`](rules/compatibility.md#at2gd) | 'aut2geod' has been removed. With appropriate code changes, use 'map.geodesy.AuthalicLatitudeConverter' instead. |
| [`CN2GD`](rules/compatibility.md#cn2gd) | 'cen2geod' has been removed. With appropriate code changes, use 'geodeticLatitudeFromGeocentric' instead. |
| [`CF2GD`](rules/compatibility.md#cf2gd) | 'cnf2geod' has been removed. With appropriate code changes, use 'map.geodesy.ConformalLatitudeConverter' instead. |
| [`IS2GD`](rules/compatibility.md#is2gd) | 'iso2geod' has been removed. With appropriate code changes, use 'map.geodesy.IsometricLatitudeConverter' instead. |
| [`PR2GD`](rules/compatibility.md#pr2gd) | 'par2geod' has been removed. With appropriate code changes, use 'geodeticLatitudeFromParametric' instead. |
| [`RC2GD`](rules/compatibility.md#rc2gd) | 'rec2geod' has been removed. With appropriate code changes, use 'map.geodesy.RectifyingLatitudeConverter' instead. |
| [`GD2AT`](rules/compatibility.md#gd2at) | 'geod2aut' has been removed. With appropriate code changes, use 'map.geodesy.AuthalicLatitudeConverter' instead. |
| [`GD2CN`](rules/compatibility.md#gd2cn) | 'geod2cen' has been removed. With appropriate code changes, use 'geocentricLatitude' instead. |
| [`GD2CF`](rules/compatibility.md#gd2cf) | 'geod2cnf' has been removed. With appropriate code changes, use 'map.geodesy.ConformalLatitudeConverter' instead. |
| [`GD2IS`](rules/compatibility.md#gd2is) | 'geod2iso' has been removed. With appropriate code changes, use 'map.geodesy.IsometricLatitudeConverter' instead. |
| [`GD2PR`](rules/compatibility.md#gd2pr) | 'geod2par' has been removed. With appropriate code changes, use 'parametricLatitude' instead. |
| [`GD2RC`](rules/compatibility.md#gd2rc) | 'geod2rec' has been removed. With appropriate code changes, use 'map.geodesy.RectifyingLatitudeConverter' instead. |
| [`DCWDT`](rules/compatibility.md#dcwdt) | 'dcwdata' has been removed. With appropriate code changes, use 'vmap0data' instead. |
| [`DCWGZ`](rules/compatibility.md#dcwgz) | 'dcwgaz' has been removed. With appropriate code changes, use 'vmap0ui' instead. |
| [`DCWRD`](rules/compatibility.md#dcwrd) | 'dcwread' has been removed. With appropriate code changes, use 'vmap0read' instead. |
| [`DCWHD`](rules/compatibility.md#dcwhd) | 'dcwrhead' has been removed. With appropriate code changes, use 'vmap0rhead' instead. |
| [`SYMBM`](rules/compatibility.md#symbm) | 'symbolm' has been removed. With appropriate code changes, use 'scatterm' instead. |
| [`TRACKUI`](rules/compatibility.md#trackui) | 'trackui' has been removed. With appropriate code changes, use 'trackg' instead. |
| [`SCIRCLUI`](rules/compatibility.md#scirclui) | 'scirclui' has been removed. With appropriate code changes, use 'scircleg' instead. |
| [`ORIGINUI`](rules/compatibility.md#originui) | 'originui' has been removed. With appropriate code changes, use 'setm' instead. |
| [`PARALLELUI`](rules/compatibility.md#parallelui) | 'parallelui' has been removed. With appropriate code changes, use 'setm' instead. |
| [`SECTORG`](rules/compatibility.md#sectorg) | 'sectorg' has been removed. With appropriate code changes, use 'scircle1' instead. |
| [`CLRMENU`](rules/compatibility.md#clrmenu) | 'clrmenu' has been removed. With appropriate code changes, use 'colormapeditor' instead. |
| [`MAPTRIM`](rules/compatibility.md#maptrim) | 'maptrim' has been removed. With appropriate code changes, use 'geocrop' or 'geoclip' instead. |
| [`SURFDIST`](rules/compatibility.md#surfdist) | 'surfdist' has been removed. With appropriate code changes, use 'distance' instead. |
| [`DEMDATAUI`](rules/compatibility.md#demdataui) | 'demdataui' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| [`VMAP0UI`](rules/compatibility.md#vmap0ui) | 'vmap0ui' has been removed. With appropriate code changes, use 'vmap0read' instead. |
| [`MFWDT`](rules/compatibility.md#mfwdt) | 'mfwdtran' has been removed. With appropriate code changes, use 'projfwd' instead. |
| [`MINVT`](rules/compatibility.md#minvt) | 'minvtran' has been removed. With appropriate code changes, use 'projinv' instead. |
| [`SDTSINFO`](rules/compatibility.md#sdtsinfo) | 'sdtsinfo' has been removed. With appropriate code changes, use 'georasterinfo' instead. |
| [`LATLON2PIX`](rules/compatibility.md#latlon2pix) | 'latlon2pix' has been removed. With appropriate code changes, use 'geographicToIntrinsic' instead. |
| [`LTLNV`](rules/compatibility.md#ltlnv) | 'ltln2val' has been removed. With appropriate code changes, use 'geointerp' instead. |
| [`MAP2PIX`](rules/compatibility.md#map2pix) | 'map2pix' has been removed. With appropriate code changes, use 'worldToIntrinsic' instead. |
| [`MAPTM`](rules/compatibility.md#maptm) | 'maptrims' has been removed. With appropriate code changes, use 'geocrop' instead. |
| [`MESHGRAT`](rules/compatibility.md#meshgrat) | 'meshgrat' has been removed. With appropriate code changes, use 'geographicGrid', 'linspace' or 'ndgrid' instead. |
| [`NANM`](rules/compatibility.md#nanm) | 'nanm' has been removed. With appropriate code changes, use 'nan' instead. |
| [`ONEM`](rules/compatibility.md#onem) | 'onem' has been removed. With appropriate code changes, use 'ones' instead. |
| [`PIX2LATLON`](rules/compatibility.md#pix2latlon) | 'pix2latlon' has been removed. With appropriate code changes, use 'intrinsicToGeographic' instead. |
| [`PIX2MAP`](rules/compatibility.md#pix2map) | 'pix2map' has been removed. With appropriate code changes, use 'intrinsicToWorld' instead. |
| [`PIXCENTERS`](rules/compatibility.md#pixcenters) | 'pixcenters' has been removed. With appropriate code changes, use 'worldGrid' or 'geographicGrid' instead. |
| [`RESZM`](rules/compatibility.md#reszm) | 'resizem' has been removed. With appropriate code changes, use 'georesize' or 'imresize' instead. |
| [`SETLTLN`](rules/compatibility.md#setltln) | 'setltln' has been removed. With appropriate code changes, use 'intrinsicToGeographic' instead. |
| [`SETPOSTN`](rules/compatibility.md#setpostn) | 'setpostn' has been removed. With appropriate code changes, use 'geographicToDiscrete' instead. |
| [`SPZER`](rules/compatibility.md#spzer) | 'spzerom' has been removed. With appropriate code changes, use 'sparse' instead. |
| [`ZEROM`](rules/compatibility.md#zerom) | 'zerom' has been removed. With appropriate code changes, use 'zeros' instead. |
| [`MDTED`](rules/compatibility.md#mdted) | 'dted' will be removed in a future release. With appropriate code changes, use 'readgeoraster' instead. |
| [`ETOPO`](rules/compatibility.md#etopo) | 'etopo' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| [`GLDEM`](rules/compatibility.md#gldem) | 'globedem' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| [`GTOPO`](rules/compatibility.md#gtopo) | 'gtopo30' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| [`SBATH`](rules/compatibility.md#sbath) | 'satbath' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| [`SDTRD`](rules/compatibility.md#sdtrd) | 'sdtsdemread' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| [`TBASE`](rules/compatibility.md#tbase) | 'tbase' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| [`USGDM`](rules/compatibility.md#usgdm) | 'usgsdem' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| [`USGKD`](rules/compatibility.md#usgkd) | 'usgs24kdem' has been removed. With appropriate code changes, use 'readgeoraster' instead. |
| [`DMCHN`](rules/compatibility.md#dmchn) | 'mimochan' has been removed. With appropriate code changes, use 'comm.MIMOChannel' instead. |
| [`DPGUPDLG`](rules/compatibility.md#dpgupdlg) | 'pagesetupdlg' has been removed. With appropriate code changes, use 'uiprintdlg' instead. |
| [`MTHDPOOL`](rules/compatibility.md#mthdpool) | 'parcluster.matlabpool' has been removed. With appropriate code changes, use 'parpool' instead. |
| [`PDECT`](rules/compatibility.md#pdect) | 'pdecont' has been removed. With appropriate code changes, use 'pdeplot' instead. |
| [`PDESF`](rules/compatibility.md#pdesf) | 'pdesurf' has been removed. With appropriate code changes, use 'pdeplot' instead. |
| [`POLYCPO`](rules/compatibility.md#polycpo) | 'polyspace.CodeProverOptions' has been removed. With appropriate code changes, use 'polyspace.Options' instead. |
| [`POLYBFO`](rules/compatibility.md#polybfo) | 'polyspace.BugFinderOptions' has been removed. With appropriate code changes, use 'polyspace.Options' instead. |
| [`PRPRE`](rules/compatibility.md#prpre) | 'printpreview' will be removed in a future release. With appropriate code changes, use 'uiprintdlg' instead. |
| [`PRDLG`](rules/compatibility.md#prdlg) | 'printdlg' will be removed in a future release. With appropriate code changes, use 'uiprintdlg' instead. |
| [`EXPSE`](rules/compatibility.md#expse) | 'exportsetupdlg' will be removed in a future release. With appropriate code changes, use 'uiexportdlg' instead. |
| [`RWA`](rules/compatibility.md#rwa) | 'radarWaveformAnalyzer' will be removed in a future release. Use 'pulseWaveformAnalyzer' instead, which is a direct replacement. |
| [`RCSFLT`](rules/compatibility.md#rcsflt) | 'rcosflt' is unsupported and has been removed. With appropriate code changes, use 'rcosdesign' instead. |
| [`RCSINE`](rules/compatibility.md#rcsine) | 'rcosine' is unsupported and has been removed. With appropriate code changes, use 'rcosdesign' instead. |
| [`SVM2SUB`](rules/compatibility.md#svm2sub) | 'Simulink.VariantManager.convertToVariant' will be removed in a future release. Use 'Simulink.VariantUtils.convertToVariantSubsystem' instead, which is a direct replacement. |
| [`SVM2SUBA`](rules/compatibility.md#svm2suba) | 'Simulink.VariantManager.convertToVariantAssemblySubsystem' will be removed in a future release. Use 'Simulink.VariantUtils.convertToVariantAssemblySubsystem' instead, which is a direct replacement. |
| [`SVMVL`](rules/compatibility.md#svmvl) | 'Simulink.VariantManager.variantLegend' will be removed in a future release. Use 'Simulink.VariantUtils.variantLegend' instead, which is a direct replacement. |
| [`SLLWARN`](rules/compatibility.md#sllwarn) | 'sllastwarning' will be removed in a future release. With appropriate code changes, use 'lastwarn' instead. |
| [`SLLERR1`](rules/compatibility.md#sllerr1) | 'sllasterror' will be removed in a future release. Use an identifier on the CATCH block instead. |
| [`SLLERR2`](rules/compatibility.md#sllerr2) | 'sllastdiagnostic' will be removed in a future release. Use an identifier on the CATCH block instead. |
| [`MOVIEVW`](rules/compatibility.md#movievw) | 'movieview' has been removed. Use 'implay' instead, which is a direct replacement. |
| [`IMAGEVW`](rules/compatibility.md#imagevw) | 'imageview' has been removed. Use 'imshow' instead, which is a direct replacement. |
| [`SOUNDVW`](rules/compatibility.md#soundvw) | 'soundview' has been removed. With appropriate code changes, use 'audioread' with 'plot' or 'sound' instead. |
| [`DWVRD`](rules/compatibility.md#dwvrd) | 'wavread' has been removed. With appropriate code changes, use 'audioread' instead. |
| [`DWVWR`](rules/compatibility.md#dwvwr) | 'wavwrite' has been removed. With appropriate code changes, use 'audiowrite' instead. |
| [`DWVFINF`](rules/compatibility.md#dwvfinf) | 'wavfinfo' has been removed. With appropriate code changes, use 'audioinfo' instead. |
| [`WMCTR`](rules/compatibility.md#wmctr) | 'webmap' and its associated function 'wmcenter' will be removed in a future release. With appropriate code changes, use 'MapCenter' property of geographic axes object instead. |
| [`WMCLS`](rules/compatibility.md#wmcls) | 'webmap' and its associated function 'wmclose' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'close' function instead. |
| [`WMLMT`](rules/compatibility.md#wmlmt) | 'webmap' and its associated function 'wmlimits' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'geolimits' function instead. |
| [`WMLIN`](rules/compatibility.md#wmlin) | 'webmap' and its associated function 'wmline' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'geoplot' function instead. |
| [`WMMKR`](rules/compatibility.md#wmmkr) | 'webmap' and its associated function 'wmmarker' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'geoiconchart' function instead. |
| [`WMPYG`](rules/compatibility.md#wmpyg) | 'webmap' and its associated function 'wmpolygon' will be removed in a future release. With appropriate code changes, use a geographic axes object, a 'geopolyshape' object, and 'geoplot' function instead. |
| [`WMPNT`](rules/compatibility.md#wmpnt) | 'webmap' and its associated function 'wmprint' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'exportgraphics' function instead. |
| [`WMRMV`](rules/compatibility.md#wmrmv) | 'webmap' and its associated function 'wmremove' will be removed in a future release. With appropriate code changes, use a geographic axes object and 'delete' function instead. |
| [`WMZOM`](rules/compatibility.md#wmzom) | 'webmap' and its associated function 'wmzoom' will be removed in a future release. With appropriate code changes, use 'ZoomLevel' property of geographic axes object instead. |
| [`BETALIK1`](rules/compatibility.md#betalik1) | 'betalik1' has been removed. With appropriate code changes, use 'betalike' instead. |
| [`SVMSMOSET`](rules/compatibility.md#svmsmoset) | 'svmsmoset' has been removed. With appropriate code changes, use 'fitcsvm' instead. |
| [`LTEFS`](rules/compatibility.md#ltefs) | 'ltehdlFramesToSamples' has been removed. Use 'whdlFramesToSamples' instead, which is a direct replacement. |
| [`LTESF`](rules/compatibility.md#ltesf) | 'ltehdlSamplesToFrames' has been removed. Use 'whdlSamplesToFrames' instead, which is a direct replacement. |
| [`VHTLTFDEM`](rules/compatibility.md#vhtltfdem) | 'wlanVHTLTFDemodulate' will be removed in a future release. With appropriate code changes, use 'wlanVHTDemodulate' instead. |
| [`VHTDATAREC`](rules/compatibility.md#vhtdatarec) | 'wlanVHTDataRecover' will be removed in a future release. With appropriate code changes, use 'wlanVHTDataBitRecover' instead. |
| [`VHTSIGAREC`](rules/compatibility.md#vhtsigarec) | 'wlanVHTSIGARecover' will be removed in a future release. With appropriate code changes, use 'wlanVHTSIGABitRecover' instead. |
| [`VHTSIGBREC`](rules/compatibility.md#vhtsigbrec) | 'wlanVHTSIGBRecover' will be removed in a future release. With appropriate code changes, use 'wlanVHTSIGBBitRecover' instead. |
| [`BLBAF`](rules/compatibility.md#blbaf) | Manually setting 'BytesAvailableFcnCount' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'bluetooth' class to set value instead. |
| [`BLTMT`](rules/compatibility.md#bltmt) | Manually setting 'Terminator' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'bluetooth' class to set value instead. |
| [`BLIBS`](rules/compatibility.md#blibs) | 'InputBufferSize' property of 'bluetooth' class will be removed in a future release. There is no simple replacement for this. |
| [`SPPSS`](rules/compatibility.md#sppss) | 'PinStatus' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'getpinstatus' method of 'serialport' class instead. |
| [`SPTMT`](rules/compatibility.md#sptmt) | Manually setting 'Terminator' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'serialport' class to set value instead. |
| [`SPIBS`](rules/compatibility.md#spibs) | 'InputBufferSize' property of 'serialport' class will be removed in a future release. There is no simple replacement for this. |
| [`TCTMT`](rules/compatibility.md#tctmt) | Manually setting 'Terminator' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'tcpclient' class to set value instead. |
| [`TCIBS`](rules/compatibility.md#tcibs) | 'InputBufferSize' property of 'tcpclient' class will be removed in a future release. There is no simple replacement for this. |
| [`TSTMT`](rules/compatibility.md#tstmt) | Manually setting 'Terminator' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'tcpserver' class to set value instead. |
| [`TSIBS`](rules/compatibility.md#tsibs) | 'InputBufferSize' property of 'tcpserver' class will be removed in a future release. There is no simple replacement for this. |
| [`UDTMT`](rules/compatibility.md#udtmt) | Manually setting 'Terminator' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'udpport' class to set value instead. |
| [`UDIBS`](rules/compatibility.md#udibs) | 'InputBufferSize' property of 'udpport' class will be removed in a future release. There is no simple replacement for this. |
| [`VSTMT`](rules/compatibility.md#vstmt) | Manually setting 'Terminator' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'visadev' class to set value instead. |
| [`VSIBS`](rules/compatibility.md#vsibs) | 'InputBufferSize' property of 'visadev' class will be removed in a future release. There is no simple replacement for this. |
| [`VSBYA`](rules/compatibility.md#vsbya) | 'BytesAvailable' property of 'visadev' class will be removed in a future release. There is no simple replacement for this. |
| [`VLSBC`](rules/compatibility.md#vlsbc) | 'BackgroundColor' property has been removed. With appropriate code changes, use 'BackgroundColor' property of the parent 'Viewer3D' class instead. |
| [`VLSCP`](rules/compatibility.md#vlscp) | 'CameraPosition' property has been removed. With appropriate code changes, use 'CameraPosition' property of the parent 'Viewer3D' class instead. |
| [`VLSCT`](rules/compatibility.md#vlsct) | 'CameraTarget' property has been removed. With appropriate code changes, use 'CameraTarget' property of the parent 'Viewer3D' class instead. |
| [`VLSCU`](rules/compatibility.md#vlscu) | 'CameraUpVector' property has been removed. With appropriate code changes, use 'CameraUpVector' property of the parent 'Viewer3D' class instead. |
| [`VLSCV`](rules/compatibility.md#vlscv) | 'CameraViewAngle' property has been removed. There is no simple replacement for this. |
| [`VLSIE`](rules/compatibility.md#vlsie) | 'InteractionsEnabled' property has been removed. With appropriate code changes, use 'Interactions' property of the parent 'Viewer3D' class instead. |
| [`VLSLT`](rules/compatibility.md#vlslt) | 'Lighting' property has been removed. With appropriate code changes, use 'Lighting' property of the parent 'Viewer3D' class instead. |
| [`VLSRD`](rules/compatibility.md#vlsrd) | 'Renderer' property has been removed. With appropriate code changes, use 'RenderingStyle' property instead. |
| [`VLSIC`](rules/compatibility.md#vlsic) | 'IsosurfaceColor' property has been removed. With appropriate code changes, use 'Colormap' property instead. |
| [`VLSSF`](rules/compatibility.md#vlssf) | 'ScaleFactors' property has been removed. With appropriate code changes, use 'Transformation' property instead. |
| [`VLSIV`](rules/compatibility.md#vlsiv) | 'Isovalue' property has been removed. With appropriate code changes, use 'IsosurfaceValue' property instead. |
| [`SESSIONQOD`](rules/compatibility.md#sessionqod) | 'queueOutputData' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'write' method of 'DataAcquisition' class instead. |
| [`SESSIONACC`](rules/compatibility.md#sessionacc) | 'addClockConnection' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'addclock' method of 'DataAcquisition' class instead. |
| [`SESSIONATC`](rules/compatibility.md#sessionatc) | 'addTriggerConnection' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'addtrigger' method of 'DataAcquisition' class instead. |
| [`SESSIONISS`](rules/compatibility.md#sessioniss) | 'inputSingleScan' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'read' method of 'DataAcquisition' class instead. |
| [`SESSIONOSS`](rules/compatibility.md#sessionoss) | 'outputSingleScan' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'write' method of 'DataAcquisition' class instead. |
| [`SESSIONSB`](rules/compatibility.md#sessionsb) | 'startBackground' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'start' method of 'DataAcquisition' class instead. |
| [`SESSIONSF`](rules/compatibility.md#sessionsf) | 'startForeground' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'write' method of 'DataAcquisition' class instead. |
| [`BLFWR`](rules/compatibility.md#blfwr) | 'fwrite' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'write' method of 'bluetooth' class instead. |
| [`BLSTR`](rules/compatibility.md#blstr) | 'scanstr' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'bluetooth' class instead. |
| [`SPSTR`](rules/compatibility.md#spstr) | 'scanstr' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'serialport' class instead. |
| [`TCSTR`](rules/compatibility.md#tcstr) | 'scanstr' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpclient' class instead. |
| [`TSSTR`](rules/compatibility.md#tsstr) | 'scanstr' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpserver' class instead. |
| [`UDSTR`](rules/compatibility.md#udstr) | 'scanstr' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'udpport' class instead. |
| [`VSSTR`](rules/compatibility.md#vsstr) | 'scanstr' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'visadev' class instead. |
| [`SESSIONAIC`](rules/compatibility.md#sessionaic) | 'addAnalogInputChannel' method of 'Session' class will be removed in a future release. Use 'addinput' of 'DataAcquisition' class instead, which is a direct replacement. |
| [`SESSIONADIC`](rules/compatibility.md#sessionadic) | 'addAudioInputChannel' method of 'Session' class will be removed in a future release. Use 'addinput' of 'DataAcquisition' class instead, which is a direct replacement. |
| [`SESSIONAOC`](rules/compatibility.md#sessionaoc) | 'addAnalogOutputChannel' method of 'Session' class will be removed in a future release. Use 'addoutput' of 'DataAcquisition' class instead, which is a direct replacement. |
| [`SESSIONADOC`](rules/compatibility.md#sessionadoc) | 'addAudioOutputChannel' method of 'Session' class will be removed in a future release. Use 'addoutput' of 'DataAcquisition' class instead, which is a direct replacement. |
| [`SESSIONCOC`](rules/compatibility.md#sessioncoc) | 'addCounterOutputChannel' method of 'Session' class will be removed in a future release. Use 'addoutput' of 'DataAcquisition' class instead, which is a direct replacement. |
| [`SESSIONFGC`](rules/compatibility.md#sessionfgc) | 'addFunctionGeneratorChannel' method of 'Session' class will be removed in a future release. Use 'addoutput' of 'DataAcquisition' class instead, which is a direct replacement. |
| [`SESSIONRC`](rules/compatibility.md#sessionrc) | 'removeChannel' method of 'Session' class will be removed in a future release. Use 'removechannel' of 'DataAcquisition' class instead, which is a direct replacement. |
| [`SESSIONRSC`](rules/compatibility.md#sessionrsc) | 'resetCounters' method of 'Session' class will be removed in a future release. Use 'resetcounters' of 'DataAcquisition' class instead, which is a direct replacement. |
| [`SESSIONCIC`](rules/compatibility.md#sessioncic) | 'addCounterInputChannel' method of 'Session' class will be removed in a future release. Use 'addinput' of 'DataAcquisition' class instead, which is a direct replacement. |
| [`SESSIONDIS`](rules/compatibility.md#sessiondis) | 'DurationInSeconds' property of 'Session' class will be removed in a future release. With appropriate code changes, specify 'Duration' as argument to 'read' or 'start' instead. |
| [`SESSIONNOS`](rules/compatibility.md#sessionnos) | 'NumberOfScans' property of 'Session' class will be removed in a future release. With appropriate code changes, specify 'NumScans' as argument to 'read' or 'start' instead. |
| [`SESSIONADC`](rules/compatibility.md#sessionadc) | 'addDigitalChannel' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'addinput' or 'addoutput' of 'DataAcquisition' class instead. |
| [`SESSIONRCON`](rules/compatibility.md#sessionrcon) | 'removeConnection' method of 'Session' class will be removed in a future release. With appropriate code changes, use 'removeclock' or 'removetrigger' of 'DataAcquisition' class instead. |
| [`SESSIONAL`](rules/compatibility.md#sessional) | 'addlistener' method of 'Session' class will be removed in a future release. With appropriate code changes, use the DataAcquisition interface and its callback properties instead. |
| [`PRINTRAS01`](rules/compatibility.md#printras01) | '-dbmpmono' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS02`](rules/compatibility.md#printras02) | '-dbmp' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS03`](rules/compatibility.md#printras03) | '-dbmp16m' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS04`](rules/compatibility.md#printras04) | '-dbmp256' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS05`](rules/compatibility.md#printras05) | '-dhdf' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS06`](rules/compatibility.md#printras06) | '-dpbm' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS07`](rules/compatibility.md#printras07) | '-dpbmraw' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS08`](rules/compatibility.md#printras08) | '-dpcxmono' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS09`](rules/compatibility.md#printras09) | '-dpcx24b' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS10`](rules/compatibility.md#printras10) | '-dpcx256' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS11`](rules/compatibility.md#printras11) | '-dpcx16' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS12`](rules/compatibility.md#printras12) | '-dpgm' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS13`](rules/compatibility.md#printras13) | '-dpgmraw' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS14`](rules/compatibility.md#printras14) | '-dppm' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTRAS15`](rules/compatibility.md#printras15) | '-dppmraw' has been removed. With appropriate code changes, use 'imwrite' instead. |
| [`PRINTPS1`](rules/compatibility.md#printps1) | '-dps' has been removed. With appropriate code changes, use '-deps' or '-dpdf' instead. |
| [`PRINTPS2`](rules/compatibility.md#printps2) | '-dpsc' has been removed. With appropriate code changes, use '-deps' or '-dpdf' instead. |
| [`PRINTPS3`](rules/compatibility.md#printps3) | '-dps2' has been removed. With appropriate code changes, use '-deps' or '-dpdf' instead. |
| [`PRINTPS4`](rules/compatibility.md#printps4) | '-dpsc2' has been removed. With appropriate code changes, use '-deps' or '-dpdf' instead. |
| [`SVFIGC`](rules/compatibility.md#svfigc) | 'compact' argument will be removed in a future release and currently has no effect. 'compact' behavior is always enabled. |
| [`LEGMOP`](rules/compatibility.md#legmop) | Syntax to call 'legend' with multiple outputs will be removed in a future release. There is no simple replacement for this. |
| [`SMTHG`](rules/compatibility.md#smthg) | 'GraphicsSmoothing' property will be removed in a future release. Graphics smoothing behavior has been enabled by default. Editing this property does not have an effect. |
| [`SMTHGF`](rules/compatibility.md#smthgf) | 'DefaultFigureGraphicsSmoothing' setting will be removed in a future release. Graphics smoothing behavior has been enabled by default. Editing this setting does not have an effect. |
| [`SMTHF`](rules/compatibility.md#smthf) | 'FontSmoothing' property will be removed in a future release. Font smoothing behavior has been enabled by default. Editing this property does not have an effect. |
| [`SMTHFA`](rules/compatibility.md#smthfa) | 'DefaultAxesFontSmoothing' setting will be removed in a future release. Font smoothing behavior has been enabled by default. Editing this setting does not have an effect. |
| [`SMTHFT`](rules/compatibility.md#smthft) | 'DefaultTextFontSmoothing' setting will be removed in a future release. Font smoothing behavior has been enabled by default. Editing this setting does not have an effect. |
| [`LINPROGD`](rules/compatibility.md#linprogd) | 'dual-simplex-legacy' algorithm for 'linprog' solver has been removed. With appropriate code changes, set 'Algorithm' value to 'dual-simplex-highs' or 'interior-point' instead. |
| [`LINPROGA`](rules/compatibility.md#linproga) | 'active-set' algorithm for 'linprog' solver has been removed. With appropriate code changes, set 'Algorithm' value to 'interior-point' or 'dual-simplex' instead. |
| [`INTLLEG1`](rules/compatibility.md#intlleg1) | 'Algorithm' option for the 'intlinprog' solver has been removed. The 'intlinprog' solver always uses the 'highs' algorithm. |
| [`INTLLEG2`](rules/compatibility.md#intlleg2) | 'BranchRule' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG3`](rules/compatibility.md#intlleg3) | 'BranchingRule' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG4`](rules/compatibility.md#intlleg4) | 'CutGeneration' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG5`](rules/compatibility.md#intlleg5) | 'CutGenMaxIter' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG6`](rules/compatibility.md#intlleg6) | 'CutMaxIterations' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG7`](rules/compatibility.md#intlleg7) | 'Heuristics' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG8`](rules/compatibility.md#intlleg8) | 'HeuristicsMaxNodes' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG9`](rules/compatibility.md#intlleg9) | 'IPPreprocess' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG10`](rules/compatibility.md#intlleg10) | 'IntegerPreprocess' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG11`](rules/compatibility.md#intlleg11) | 'TolInteger' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG12`](rules/compatibility.md#intlleg12) | 'IntegerTolerance' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG13`](rules/compatibility.md#intlleg13) | 'LPMaxIter' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG14`](rules/compatibility.md#intlleg14) | 'LPMaxIterations' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG15`](rules/compatibility.md#intlleg15) | 'TolFunLP' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG16`](rules/compatibility.md#intlleg16) | 'LPOptimalityTolerance' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG17`](rules/compatibility.md#intlleg17) | 'NodeSelection' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG18`](rules/compatibility.md#intlleg18) | 'RelObjThreshold' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG19`](rules/compatibility.md#intlleg19) | 'ObjectiveImprovementThreshold' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG20`](rules/compatibility.md#intlleg20) | 'RootLPAlgorithm' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG21`](rules/compatibility.md#intlleg21) | 'RootLPMaxIter' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLLEG22`](rules/compatibility.md#intlleg22) | 'RootLPMaxIterations' option for the 'intlinprog' solver has been removed. There is no simple replacement for this. |
| [`INTLPOUTA`](rules/compatibility.md#intlpouta) | 'algorithm' field of the 'intlinprog' solution process summary has been removed. 'intlinprog' always uses the 'highs' algorithm. |
| [`COEFFS`](rules/compatibility.md#coeffs) | The property 'FilterSpecification' has been removed. |
| [`COEFF1`](rules/compatibility.md#coeff1) | The property 'FirstFilterCoefficients' has been removed. |
| [`COEFF2`](rules/compatibility.md#coeff2) | The property 'SecondFilterCoefficients' has been removed. |
| [`COEFF3`](rules/compatibility.md#coeff3) | The property 'ThirdFilterCoefficients' has been removed. |
| [`COEFFD1`](rules/compatibility.md#coeffd1) | The property 'FirstFilterCoefficientsDataType' has been removed. |
| [`COEFFD2`](rules/compatibility.md#coeffd2) | The property 'SecondFilterCoefficientsDataType' has been removed. |
| [`COEFFD3`](rules/compatibility.md#coeffd3) | The property 'ThirdFilterCoefficientsDataType' has been removed. |
| [`COEFFC1`](rules/compatibility.md#coeffc1) | The property 'CustomFirstFilterCoefficientsDataType' has been removed. |
| [`COEFFC2`](rules/compatibility.md#coeffc2) | The property 'CustomSecondFilterCoefficientsDataType' has been removed. |
| [`COEFFC3`](rules/compatibility.md#coeffc3) | The property 'CustomThirdFilterCoefficientsDataType' has been removed. |
| [`READSZK`](rules/compatibility.md#readszk) | The property 'KeyValueLimit' has been removed. |
| [`READSZR`](rules/compatibility.md#readszr) | The property 'RowsPerRead' has been removed. |
| [`OPTMOPT`](rules/compatibility.md#optmopt) | solve(PROBLEM, OPTIONS) has been removed. Use solve(PROBLEM, 'Options', OPTIONS) instead. |
| [`OPTMSLV`](rules/compatibility.md#optmslv) | solve(PROBLEM, SOLVER) has been removed. Use solve(PROBLEM, 'Solver', SOLVER) instead. |
| [`OPTMNVP`](rules/compatibility.md#optmnvp) | solve(PROBLEM, SOLVER, OPTIONS) has been removed. Use solve(PROBLEM, 'Solver', SOLVER, 'Options', OPTIONS) instead. |
| [`POLYREP`](rules/compatibility.md#polyrep) | Use 'MergedReporting' instead of 'Reporting' for polyspace.Options. |
| [`POLYCS`](rules/compatibility.md#polycs) | Use 'MergedComputingSettings' instead of 'ComputingSettings' for polyspace.Options. |
| [`ADTPATH`](rules/compatibility.md#adtpath) | 'path' will be removed in a future release. Use 'trajectory' instead. |
| [`FPRENAME`](rules/compatibility.md#fprename) | Input argument 'fixpoint' will be removed in a future release. Use 'fixedpoint' instead. |
| [`XPCRENAME`](rules/compatibility.md#xpcrename) | Input argument 'xpc' will be removed in a future release. Use 'slrealtime' instead. |
| [`SLRTRENAME`](rules/compatibility.md#slrtrename) | Input argument 'slrt' will be removed in a future release. Use 'slrealtime' instead. |
| [`PSRENAME`](rules/compatibility.md#psrename) | Input argument 'powersys' will be removed in a future release. Use 'sps' instead. |
| [`DCRENAME`](rules/compatibility.md#dcrename) | Input argument 'distcomp' will be removed in a future release. Use 'parallel' instead. |
| [`SERENAME`](rules/compatibility.md#serename) | Input argument 'simevents' will be removed in a future release. Use 'slde' instead. |
| [`HHCNA`](rules/compatibility.md#hhcna) | Input argument 'North America' has been removed. Use 'hrn:here:data::olp-here-had:here-hdlm-protobuf-na-2' instead. |
| [`HHCWE`](rules/compatibility.md#hhcwe) | Input argument 'Western Europe' has been removed. Use 'hrn:here:data::olp-here-had:here-hdlm-protobuf-weu-2' instead. |
| [`INSTHWB`](rules/compatibility.md#insthwb) | 'instrhwinfo('bluetooth',...)' will be removed in a future release. With appropriate code changes, use 'bluetoothlist' instead. |
| [`INSTHWT`](rules/compatibility.md#insthwt) | 'instrhwinfo('tcpip')' will be removed in a future release. There is no simple replacement for this. |
| [`INSTHWU`](rules/compatibility.md#insthwu) | 'instrhwinfo('udp')' will be removed in a future release. There is no simple replacement for this. |
| [`CNNCGA`](rules/compatibility.md#cnncga) | 'cnncodegen' with 'targetlib' as 'arm-compute' has been removed. With appropriate code changes, use 'codegen' instead. |
| [`CNNCGT`](rules/compatibility.md#cnncgt) | 'cnncodegen' with 'targetlib' as 'tensorrt' has been removed. With appropriate code changes, use 'codegen' instead. |
| [`CNNCGC`](rules/compatibility.md#cnncgc) | 'cnncodegen' with 'targetlib' as 'cudnn' has been removed. With appropriate code changes, use 'codegen' instead. |
| [`CNNCGM`](rules/compatibility.md#cnncgm) | 'cnncodegen' with 'targetlib' as 'mkldnn' has been removed. With appropriate code changes, use 'codegen' instead. |
| [`MAOUTL`](rules/compatibility.md#maoutl) | 'mapoutline' with referencing matrix has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MAWFW`](rules/compatibility.md#mawfw) | 'worldfilewrite' with referencing matrix has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MAARMT`](rules/compatibility.md#maarmt) | 'areamat' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MAFDM`](rules/compatibility.md#mafdm) | 'findm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MAPFIL`](rules/compatibility.md#mapfil) | 'mapprofile' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MAGRDNT`](rules/compatibility.md#magrdnt) | 'gradientm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MALOS2`](rules/compatibility.md#malos2) | 'los2' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MAVWSH`](rules/compatibility.md#mavwsh) | 'viewshed' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MANORG`](rules/compatibility.md#manorg) | 'neworig' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MACTRM`](rules/compatibility.md#mactrm) | 'contourm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MACTFM`](rules/compatibility.md#mactfm) | 'contourfm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MACT3M`](rules/compatibility.md#mact3m) | 'contour3m' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MASHM`](rules/compatibility.md#mashm) | 'meshm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MASHL`](rules/compatibility.md#mashl) | 'meshlsrm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MAGTFW`](rules/compatibility.md#magtfw) | 'geotiffwrite' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MAFLTM`](rules/compatibility.md#mafltm) | 'filterm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MAVC2MX`](rules/compatibility.md#mavc2mx) | 'vec2mtx' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MAIMBED`](rules/compatibility.md#maimbed) | 'imbedm' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`MAG2I`](rules/compatibility.md#mag2i) | 'grid2image' with referencing matrix or referencing vector has been removed. With appropriate code changes, use a geographic or map raster reference object as input instead. |
| [`WFLRD`](rules/compatibility.md#wflrd) | 'worldfileread(worldFileName)' has been removed. With appropriate code changes, use 'worldfileread(worldFileName, coordinateSystemType, rasterSize)' instead. |
| [`EGMGD`](rules/compatibility.md#egmgd) | 'egm96geoid(SAMPLEFACTOR,...)' has been removed. With appropriate code changes, use 'egm96geoid(R)' instead, where R is a geographic raster reference object. |
| [`INSTHWS`](rules/compatibility.md#insthws) | 'instrhwinfo('serial')' will be removed in a future release. With appropriate code changes, use 'serialportlist' instead. |
| [`INSTHWSP`](rules/compatibility.md#insthwsp) | 'instrhwinfo('serialport')' will be removed in a future release. With appropriate code changes, use 'serialportlist' instead. |
| [`INSTHWV`](rules/compatibility.md#insthwv) | 'instrhwinfo('visa')' will be removed in a future release. With appropriate code changes, use 'visadevlist' instead. |
| [`OPGLD`](rules/compatibility.md#opgld) | 'opengl('data')' has been removed. With appropriate code changes, use 'rendererinfo' instead. |
| [`OPGLO`](rules/compatibility.md#opglo) | 'opengl' has been removed. There is no simple replacement for this. |
| [`PMRTM1`](rules/compatibility.md#pmrtm1) | 'propagationModel('raytracing-image-method')' syntax has been removed. With appropriate code changes, use 'propagationModel('raytracing', 'Method', 'image')' syntax instead. |
| [`PMRTM2`](rules/compatibility.md#pmrtm2) | 'propagationModel('raytracing-imagemethod')' syntax has been removed. With appropriate code changes, use 'propagationModel('raytracing', 'Method', 'image')' syntax instead. |
| [`PMRTM3`](rules/compatibility.md#pmrtm3) | 'propagationModel('raytracingimage-method')' syntax has been removed. With appropriate code changes, use 'propagationModel('raytracing', 'Method', 'image')' syntax instead. |
| [`PMRTM4`](rules/compatibility.md#pmrtm4) | 'propagationModel('raytracingimagemethod')' syntax has been removed. With appropriate code changes, use 'propagationModel('raytracing', 'Method' , 'image')' syntax instead. |
| [`INSTHWG`](rules/compatibility.md#insthwg) | 'instrhwinfo('gpib')' will be removed in a future release. With appropriate code changes, use 'visadevlist' instead. |
| [`BLFOP`](rules/compatibility.md#blfop) | 'fopen' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'bluetooth' constructor instead. |
| [`SPFOP`](rules/compatibility.md#spfop) | 'fopen' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'serialport' constructor instead. |
| [`TCFOP`](rules/compatibility.md#tcfop) | 'fopen' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'tcpclient' constructor instead. |
| [`TSFOP`](rules/compatibility.md#tsfop) | 'fopen' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'tcpserver' constructor instead. |
| [`UDFOP`](rules/compatibility.md#udfop) | 'fopen' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'udpport' constructor instead. |
| [`VSFOP`](rules/compatibility.md#vsfop) | 'fopen' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'visadev' constructor instead. |
| [`SPFWR`](rules/compatibility.md#spfwr) | 'fwrite' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'write' method of 'serialport' class instead. |
| [`TCFWR`](rules/compatibility.md#tcfwr) | 'fwrite' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'write' method of 'tcpclient' class instead. |
| [`TSFWR`](rules/compatibility.md#tsfwr) | 'fwrite' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'write' method of 'tcpserver' class instead. |
| [`UDFWR`](rules/compatibility.md#udfwr) | 'fwrite' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'write' method of 'udpport' class instead. |
| [`VSFWR`](rules/compatibility.md#vsfwr) | 'fwrite' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'write' method of 'visadev' class instead. |
| [`BLFRD`](rules/compatibility.md#blfrd) | 'fread' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'read' method of 'bluetooth' class instead. |
| [`SPFRD`](rules/compatibility.md#spfrd) | 'fread' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'read' method of 'serialport' class instead. |
| [`TCFRD`](rules/compatibility.md#tcfrd) | 'fread' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'read' method of 'tcpclient' class instead. |
| [`TSFRD`](rules/compatibility.md#tsfrd) | 'fread' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'read' method of 'tcpserver' class instead. |
| [`UDFRD`](rules/compatibility.md#udfrd) | 'fread' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'read' method of 'udpport' class instead. |
| [`VSFRD`](rules/compatibility.md#vsfrd) | 'fread' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'read' method of 'visadev' class instead. |
| [`BLFPR`](rules/compatibility.md#blfpr) | 'fprintf' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'bluetooth' class instead. |
| [`SPFPR`](rules/compatibility.md#spfpr) | 'fprintf' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'serialport' class instead. |
| [`TCFPR`](rules/compatibility.md#tcfpr) | 'fprintf' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'tcpclient' class instead. |
| [`TSFPR`](rules/compatibility.md#tsfpr) | 'fprintf' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'tcpserver' class instead. |
| [`UDFPR`](rules/compatibility.md#udfpr) | 'fprintf' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'udpport' class instead. |
| [`VSFPR`](rules/compatibility.md#vsfpr) | 'fprintf' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'writeline' method of 'visadev' class instead. |
| [`BLFSF`](rules/compatibility.md#blfsf) | 'fscanf' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'bluetooth' class instead. |
| [`SPFSF`](rules/compatibility.md#spfsf) | 'fscanf' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'serialport' class instead. |
| [`TCFSF`](rules/compatibility.md#tcfsf) | 'fscanf' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpclient' class instead. |
| [`TSFSF`](rules/compatibility.md#tsfsf) | 'fscanf' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpserver' class instead. |
| [`UDFSF`](rules/compatibility.md#udfsf) | 'fscanf' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'udpport' class instead. |
| [`VSFSF`](rules/compatibility.md#vsfsf) | 'fscanf' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'visadev' class instead. |
| [`BLFGL`](rules/compatibility.md#blfgl) | 'fgetl' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'bluetooth' class instead. |
| [`SPFGL`](rules/compatibility.md#spfgl) | 'fgetl' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'serialport' class instead. |
| [`TCFGL`](rules/compatibility.md#tcfgl) | 'fgetl' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpclient' class instead. |
| [`TSFGL`](rules/compatibility.md#tsfgl) | 'fgetl' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpserver' class instead. |
| [`UDFGL`](rules/compatibility.md#udfgl) | 'fgetl' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'udpport' class instead. |
| [`VSFGL`](rules/compatibility.md#vsfgl) | 'fgetl' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'visadev' class instead. |
| [`BLFGT`](rules/compatibility.md#blfgt) | 'fgets' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'bluetooth' class instead. |
| [`SPFGT`](rules/compatibility.md#spfgt) | 'fgets' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'serialport' class instead. |
| [`TCFGT`](rules/compatibility.md#tcfgt) | 'fgets' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpclient' class instead. |
| [`TSFGT`](rules/compatibility.md#tsfgt) | 'fgets' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'tcpserver' class instead. |
| [`UDFGT`](rules/compatibility.md#udfgt) | 'fgets' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'udpport' class instead. |
| [`VSFGT`](rules/compatibility.md#vsfgt) | 'fgets' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'readline' method of 'visadev' class instead. |
| [`BLFLI`](rules/compatibility.md#blfli) | 'flushinput' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'bluetooth' class instead. |
| [`SPFLI`](rules/compatibility.md#spfli) | 'flushinput' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'serialport' class instead. |
| [`TCFLI`](rules/compatibility.md#tcfli) | 'flushinput' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'tcpclient' class instead. |
| [`TSFLI`](rules/compatibility.md#tsfli) | 'flushinput' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'tcpserver' class instead. |
| [`UDFLI`](rules/compatibility.md#udfli) | 'flushinput' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'udpport' class instead. |
| [`VSFLI`](rules/compatibility.md#vsfli) | 'flushinput' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'visadev' class instead. |
| [`BLFCS`](rules/compatibility.md#blfcs) | 'fclose' method of 'bluetooth' class will be removed in a future release. There is no simple replacement for this. |
| [`SPFCS`](rules/compatibility.md#spfcs) | 'fclose' method of 'serialport' class will be removed in a future release. There is no simple replacement for this. |
| [`TCFCS`](rules/compatibility.md#tcfcs) | 'fclose' method of 'tcpclient' class will be removed in a future release. There is no simple replacement for this. |
| [`TSFCS`](rules/compatibility.md#tsfcs) | 'fclose' method of 'tcpserver' class will be removed in a future release. There is no simple replacement for this. |
| [`UDFCS`](rules/compatibility.md#udfcs) | 'fclose' method of 'udpport' class will be removed in a future release. There is no simple replacement for this. |
| [`VSFCS`](rules/compatibility.md#vsfcs) | 'fclose' method of 'visadev' class will be removed in a future release. There is no simple replacement for this. |
| [`SPBBW`](rules/compatibility.md#spbbw) | 'binblockwrite' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'writebinblock' method of 'serialport' class instead. |
| [`TCBBW`](rules/compatibility.md#tcbbw) | 'binblockwrite' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'writebinblock' method of 'tcpclient' class instead. |
| [`TSBBW`](rules/compatibility.md#tsbbw) | 'binblockwrite' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'writebinblock' method of 'tcpserver' class instead. |
| [`VSBBW`](rules/compatibility.md#vsbbw) | 'binblockwrite' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'writebinblock' method of 'visadev' class instead. |
| [`SPBBR`](rules/compatibility.md#spbbr) | 'binblockread' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'readbinblock' method of 'serialport' class instead. |
| [`TCBBR`](rules/compatibility.md#tcbbr) | 'binblockread' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'readbinblock' method of 'tcpclient' class instead. |
| [`TSBBR`](rules/compatibility.md#tsbbr) | 'binblockread' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'readbinblock' method of 'tcpserver' class instead. |
| [`VSBBR`](rules/compatibility.md#vsbbr) | 'binblockread' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'readbinblock' method of 'visadev' class instead. |
| [`TCQRY`](rules/compatibility.md#tcqry) | 'query' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'writeread' method of 'tcpclient' class instead. |
| [`VSQRY`](rules/compatibility.md#vsqry) | 'query' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'writeread' method of 'visadev' class instead. |
| [`VSCRD`](rules/compatibility.md#vscrd) | 'clrdevice' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'visadev' class instead. |
| [`VSSPL`](rules/compatibility.md#vsspl) | 'spoll' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'visastatus' method of 'visadev' class instead. |
| [`VSTGR`](rules/compatibility.md#vstgr) | 'trigger' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'visatrigger' method of 'visadev' class instead. |
| [`SPBAF`](rules/compatibility.md#spbaf) | Manually setting 'BytesAvailableFcnCount' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'serialport' class to set value instead. |
| [`TCBAF`](rules/compatibility.md#tcbaf) | Manually setting 'BytesAvailableFcnCount' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpclient' class to set value instead. |
| [`TSBAF`](rules/compatibility.md#tsbaf) | Manually setting 'BytesAvailableFcnCount' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpserver' class to set value instead. |
| [`UDBAF`](rules/compatibility.md#udbaf) | Manually setting 'BytesAvailableFcnCount' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'udpport' class to set value instead. |
| [`BLBAN`](rules/compatibility.md#blban) | Manually setting 'BytesAvailableFcn' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'bluetooth' class to set value instead. |
| [`SPBAN`](rules/compatibility.md#spban) | Manually setting 'BytesAvailableFcn' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'serialport' class to set value instead. |
| [`TCBAN`](rules/compatibility.md#tcban) | Manually setting 'BytesAvailableFcn' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpclient' class to set value instead. |
| [`TSBAN`](rules/compatibility.md#tsban) | Manually setting 'BytesAvailableFcn' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpserver' class to set value instead. |
| [`UDBAN`](rules/compatibility.md#udban) | Manually setting 'BytesAvailableFcn' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'udpport' class to set value instead. |
| [`BLBAM`](rules/compatibility.md#blbam) | Manually setting 'BytesAvailableFcnMode' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'bluetooth' class to set value instead. |
| [`SPBAM`](rules/compatibility.md#spbam) | Manually setting 'BytesAvailableFcnMode' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'serialport' class to set value instead. |
| [`TCBAM`](rules/compatibility.md#tcbam) | Manually setting 'BytesAvailableFcnMode' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpclient' class to set value instead. |
| [`TSBAM`](rules/compatibility.md#tsbam) | Manually setting 'BytesAvailableFcnMode' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'tcpserver' class to set value instead. |
| [`UDBAM`](rules/compatibility.md#udbam) | Manually setting 'BytesAvailableFcnMode' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'udpport' class to set value instead. |
| [`BLBAB`](rules/compatibility.md#blbab) | 'BytesAvailable' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'NumBytesAvailable' property of 'bluetooth' class instead. |
| [`SPBAB`](rules/compatibility.md#spbab) | 'BytesAvailable' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'NumBytesAvailable' property of 'serialport' class instead. |
| [`TCBAB`](rules/compatibility.md#tcbab) | 'BytesAvailable' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'NumBytesAvailable' property of 'tcpclient' class instead. |
| [`TSBAB`](rules/compatibility.md#tsbab) | 'BytesAvailable' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'NumBytesAvailable' property of 'tcpserver' class instead. |
| [`UDBAB`](rules/compatibility.md#udbab) | 'BytesAvailable' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'NumBytesAvailable' property of 'udpport' class instead. |
| [`BLEFN`](rules/compatibility.md#blefn) | 'ErrorFcn' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'bluetooth' class instead. |
| [`SPEFN`](rules/compatibility.md#spefn) | 'ErrorFcn' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'serialport' class instead. |
| [`TCEFN`](rules/compatibility.md#tcefn) | 'ErrorFcn' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'tcpclient' class instead. |
| [`TSEFN`](rules/compatibility.md#tsefn) | 'ErrorFcn' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'tcpserver' class instead. |
| [`UDEFN`](rules/compatibility.md#udefn) | 'ErrorFcn' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'udpport' class instead. |
| [`VSEFN`](rules/compatibility.md#vsefn) | 'ErrorFcn' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'ErrorOccurredFcn' property of 'visadev' class instead. |
| [`BLREN`](rules/compatibility.md#blren) | 'RemoteName' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'Name' property of 'bluetooth' class instead. |
| [`BLRID`](rules/compatibility.md#blrid) | 'RemoteID' property of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'Address' property of 'bluetooth' class instead. |
| [`VSPSS`](rules/compatibility.md#vspss) | 'PinStatus' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'getpinstatus' method of 'visadev' class instead. |
| [`SPDTR`](rules/compatibility.md#spdtr) | 'DataTerminalReady' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'setDTR' method of 'serialport' class instead. |
| [`VSDTR`](rules/compatibility.md#vsdtr) | 'DataTerminalReady' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'setDTR' method of 'visadev' class instead. |
| [`SPRTS`](rules/compatibility.md#sprts) | 'RequestToSend' property of 'serialport' class will be removed in a future release. With appropriate code changes, use 'setRTS' method of 'serialport' class instead. |
| [`VSRTS`](rules/compatibility.md#vsrts) | 'RequestToSend' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'setRTS' method of 'visadev' class instead. |
| [`TCNTR`](rules/compatibility.md#tcntr) | 'NetworkRole' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'tcpclient' constructor instead. |
| [`TSNTR`](rules/compatibility.md#tsntr) | 'NetworkRole' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'tcpserver' constructor instead. |
| [`TCTDY`](rules/compatibility.md#tctdy) | 'TransferDelay' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'EnableTransferDelay' property of 'tcpclient' class instead. |
| [`TCRPT`](rules/compatibility.md#tcrpt) | 'RemotePort' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'Port' property of 'tcpclient' class instead. |
| [`TSRPT`](rules/compatibility.md#tsrpt) | 'RemotePort' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ServerPort' property of 'tcpserver' class instead. |
| [`TCRHT`](rules/compatibility.md#tcrht) | 'RemoteHost' property of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'Address' property of 'tcpclient' class instead. |
| [`TSRHT`](rules/compatibility.md#tsrht) | 'RemoteHost' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ClientAddress' property of 'tcpserver' class instead. |
| [`TSLHT`](rules/compatibility.md#tslht) | 'LocalHost' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ServerAddress' property of 'tcpserver' class instead. |
| [`TSLPM`](rules/compatibility.md#tslpm) | 'LocalPortMode' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ServerAddress' property of 'tcpserver' class instead. |
| [`TSLPT`](rules/compatibility.md#tslpt) | 'LocalPort' property of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'ServerAddress' property of 'tcpserver' class instead. |
| [`UDDTM`](rules/compatibility.md#uddtm) | 'DatagramTerminateMode' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'udpport' constructor instead. |
| [`UDODP`](rules/compatibility.md#udodp) | 'OutputDatagramPacketSize' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'OutputDatagramSize' property of 'udpport' class instead. |
| [`VSEMD`](rules/compatibility.md#vsemd) | 'EOSMode' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method and 'EOIMode' property of 'visadev' class instead. |
| [`VSECC`](rules/compatibility.md#vsecc) | 'EOSCharCode' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'configureTerminator' method of 'visadev' class instead. |
| [`VSMID`](rules/compatibility.md#vsmid) | 'ManufacturerID' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'VendorID' property of 'visadev' class instead. |
| [`VSMLC`](rules/compatibility.md#vsmlc) | 'ModelCode' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'ProductID' property of 'visadev' class instead. |
| [`BLFLO`](rules/compatibility.md#blflo) | 'flushoutput' method of 'bluetooth' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'bluetooth' class instead. |
| [`SPFLO`](rules/compatibility.md#spflo) | 'flushoutput' method of 'serialport' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'serialport' class instead. |
| [`TCFLO`](rules/compatibility.md#tcflo) | 'flushoutput' method of 'tcpclient' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'tcpclient' class instead. |
| [`TSFLO`](rules/compatibility.md#tsflo) | 'flushoutput' method of 'tcpserver' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'tcpserver' class instead. |
| [`UDFLO`](rules/compatibility.md#udflo) | 'flushoutput' method of 'udpport' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'udpport' class instead. |
| [`VSFLO`](rules/compatibility.md#vsflo) | 'flushoutput' method of 'visadev' class will be removed in a future release. With appropriate code changes, use 'flush' method of 'visadev' class instead. |
| [`UDDRF`](rules/compatibility.md#uddrf) | 'DatagramReceivedFcn' property of 'udpport' class will be removed in a future release. With appropriate code changes, use 'configureCallback' method of 'udpport' class instead. |
| [`VSRSN`](rules/compatibility.md#vsrsn) | 'RsrcName' property of 'visadev' class will be removed in a future release. With appropriate code changes, use 'ResourceName' property of 'visadev' class instead. |
| [`ELEVT`](rules/compatibility.md#elevt) | 'elevation' has been removed. With appropriate code changes, use 'geodetic2aer' instead. |
| [`INSTHWI2C`](rules/compatibility.md#insthwi2c) | 'instrhwinfo('i2c')' will be removed in a future release. With appropriate code changes, use 'ni845xlist' or 'aardvarklist' instead. |

### Forward Compatibility (7 checks)

| Check ID | Message |
| -------- | ------- |
| [`FCLEN`](rules/compatibility.md#fclen) | Identifiers longer than 63 characters are not supported before R2025a. |
| [`FCCPV`](rules/compatibility.md#fccpv) | Class property validation is not available before R2017a. |
| [`FCDQS`](rules/compatibility.md#fcdqs) | Double-quoted strings are not available before R2017a. Use character vectors for scalar strings or cell arrays of character vectors for string arrays. |
| [`FCFAV`](rules/compatibility.md#fcfav) | Function argument validation is not available before R2019b. |
| [`FCHBL`](rules/compatibility.md#fchbl) | Hexadecimal and binary literals are not available before R2019b. Use 'hex2dec' and 'bin2dec' instead. |
| [`FCLFS`](rules/compatibility.md#fclfs) | Local functions in a script are not available before R2016b. |
| [`FCNVA`](rules/compatibility.md#fcnva) | Name=Value syntax is not available before R2021a. Use comma-separated syntax instead. |

### Behavior Changes (905 checks)

| Check ID | Message |
| -------- | ------- |
| [`SHVAI`](rules/compatibility.md#shvai) | Explicitly define shared variables in the parent function before calling the nested function. MATLAB does not share uninitialized variables between a nested function and the parent function. |
| [`LENEMP`](rules/compatibility.md#lenemp) | Passing in text with no characters will omit the object from appearing in the legend. To revert to the old behavior, use a whitespace character instead of text with no characters. |
| [`INTRPC`](rules/compatibility.md#intrpc) | 'interp1(...,'cubic')' changed in R2020b to perform cubic convolution. To continue using shape-preserving piecewise cubic interpolation, use 'interp1(...,'pchip')' instead. |
| [`IDISVARHIGH`](rules/compatibility.md#idisvarhigh) | Variable must be explicitly defined before first use. In some cases, the definition was not required in previous releases, but it is now required. |
| [`LEGPVPAIR`](rules/compatibility.md#legpvpair) | 'legend' has changed and might interpret the name of an argument as a legend property instead of a label. |
| [`CLBGEN`](rules/compatibility.md#clbgen) | Starting in R2020a, interfaces created by 'clibgen.generateLibraryDefinition' return clib.array object instead of the equivalent MATLAB array for primitive types. |
| [`CLBBLD`](rules/compatibility.md#clbbld) | Starting in R2020a, interfaces created by 'clibgen.buildInterface' return clib.array object instead of the equivalent MATLAB array for primitive types. |
| [`COLMP`](rules/compatibility.md#colmp) | In R2019a and previous releases, the default colormap size is 64. Starting in R2019b, colormaps have 256 colors by default. |
| [`FDTAG`](rules/compatibility.md#fdtag) | 'findall' with 'Exploration.Pan', 'Exploration.ZoomIn', etc. might return empty because the data exploration buttons have moved from the figure toolbar to the axes toolbar. |
| [`NSTIMP`](rules/compatibility.md#nstimp) | Nested functions now inherit import statements from this parent function. |
| [`IDISVARLOW`](rules/compatibility.md#idisvarlow) | To avoid a potential conflict with functions on the path, explicitly define the variable before indexing into it. |
| [`WEBBEHAVE`](rules/compatibility.md#webbehave) | The 'web' function now opens external sites in your system browser by default. |
| [`GLGRI`](rules/compatibility.md#glgri) | Starting R2021a, the second output of 'geoloc2grid' is a geographic raster reference object instead of a referencing vector. |
| [`V2MTX`](rules/compatibility.md#v2mtx) | Starting R2021a, the second output of 'vec2mtx' is a geographic raster reference object instead of a referencing vector. |
| [`PTCLO`](rules/compatibility.md#ptclo) | Changing the axes LineStyleOrder or ColorOrder properties of an existing chart now affects the chart immediately. |
| [`PTDLO`](rules/compatibility.md#ptdlo) | Specifying multiple line styles in the axes LineStyleOrder might result in charts that render differently than in the previous releases. |
| [`JAPIEXT1`](rules/compatibility.md#japiext1) | 'com.teamdev.jxbrowser' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT4`](rules/compatibility.md#japiext4) | 'javax.security.auth' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT5`](rules/compatibility.md#japiext5) | 'javax.transaction.xa' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT7`](rules/compatibility.md#japiext7) | 'org.apache.el' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT8`](rules/compatibility.md#japiext8) | 'org.apache.juli' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT9`](rules/compatibility.md#japiext9) | 'org.apache.tomcat' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT10`](rules/compatibility.md#japiext10) | 'org.apache.xmlrpc' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT11`](rules/compatibility.md#japiext11) | 'org.jboss.netty' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT14`](rules/compatibility.md#japiext14) | 'org.ros.actionlib' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT15`](rules/compatibility.md#japiext15) | 'org.ros.address' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT16`](rules/compatibility.md#japiext16) | 'org.ros.concurrent' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT17`](rules/compatibility.md#japiext17) | 'org.ros.exception' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT18`](rules/compatibility.md#japiext18) | 'org.ros.gradle_plugins' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT19`](rules/compatibility.md#japiext19) | 'org.ros.internal' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT21`](rules/compatibility.md#japiext21) | 'org.ros.master' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT22`](rules/compatibility.md#japiext22) | 'org.ros.message' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT23`](rules/compatibility.md#japiext23) | 'org.ros.namespace' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT24`](rules/compatibility.md#japiext24) | 'org.ros.node' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT31`](rules/compatibility.md#japiext31) | 'org.apache.commons.codec' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT35`](rules/compatibility.md#japiext35) | 'org.apache.commons.io' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT36`](rules/compatibility.md#japiext36) | 'org.apache.commons.lang' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT38`](rules/compatibility.md#japiext38) | 'org.apache.commons.logging' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT39`](rules/compatibility.md#japiext39) | 'org.apache.commons.math3' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT40`](rules/compatibility.md#japiext40) | 'org.apache.commons.net' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT41`](rules/compatibility.md#japiext41) | 'org.apache.http' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT42`](rules/compatibility.md#japiext42) | 'org.apache.log4j' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT43`](rules/compatibility.md#japiext43) | 'org.apache.xerces' Java package and subpackages will be removed in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`CLBARY`](rules/compatibility.md#clbary) | Starting in R2020a, clib.array object is the default return value, instead of the equivalent MATLAB array for primitive types. Notify your user to update code to use clib arrays. To revert to the old behavior, call 'clibgen.generateLibraryDefinition' or 'clibgen.buildInterface' with the 'ReturnCArrays' argument set to false. |
| [`JAPIEXT20`](rules/compatibility.md#japiext20) | 'org.ros.master' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT25`](rules/compatibility.md#japiext25) | 'org.ros.rosjava_geometry' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT26`](rules/compatibility.md#japiext26) | 'org.ros.tf2' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT27`](rules/compatibility.md#japiext27) | 'org.ros.time' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT28`](rules/compatibility.md#japiext28) | 'org.xbill.DNS' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT29`](rules/compatibility.md#japiext29) | 'org.jmol.quantum' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT32`](rules/compatibility.md#japiext32) | 'ice.pilots.notsupported' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT33`](rules/compatibility.md#japiext33) | 'ice.pilots.mathml' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT34`](rules/compatibility.md#japiext34) | 'com.drew.metadata' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT37`](rules/compatibility.md#japiext37) | 'ice.pilots.domviewer' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT44`](rules/compatibility.md#japiext44) | 'cryptix.provider.mode' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT47`](rules/compatibility.md#japiext47) | 'ice.pilots.pdf' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT50`](rules/compatibility.md#japiext50) | 'org.dom4j.swing' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT54`](rules/compatibility.md#japiext54) | 'opennlp.tools.dictionary' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT55`](rules/compatibility.md#japiext55) | 'ice.util.alg' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT56`](rules/compatibility.md#japiext56) | 'org.jmol.multitouch' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT62`](rules/compatibility.md#japiext62) | 'org.jmol.minimize' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT63`](rules/compatibility.md#japiext63) | 'ice.util.awt' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT64`](rules/compatibility.md#japiext64) | 'schemaorg_apache_xmlbeans.system.s8C3F193EE11A2F798ACF65489B9E6078' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT65`](rules/compatibility.md#japiext65) | 'opennlp.tools.stemmer' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT66`](rules/compatibility.md#japiext66) | 'opennlp.tools.ngram' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT67`](rules/compatibility.md#japiext67) | 'org.jsoup.select' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT74`](rules/compatibility.md#japiext74) | 'org.drizzle.jdbc' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT75`](rules/compatibility.md#japiext75) | 'org.jmol.bspt' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT80`](rules/compatibility.md#japiext80) | 'ice.pilots.jmf' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT82`](rules/compatibility.md#japiext82) | 'thredds.inventory.partition' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT84`](rules/compatibility.md#japiext84) | 'de.l3s.boilerpipe' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT85`](rules/compatibility.md#japiext85) | 'ice.pilots.es' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT88`](rules/compatibility.md#japiext88) | 'org.bouncycastle.pkix' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT90`](rules/compatibility.md#japiext90) | 'org.dom4j.xpath' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT91`](rules/compatibility.md#japiext91) | 'ice.pilots.text' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT92`](rules/compatibility.md#japiext92) | 'thredds.cataloggen.datasetenhancer' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT96`](rules/compatibility.md#japiext96) | 'ice.util.memory' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT97`](rules/compatibility.md#japiext97) | 'org.jmol.translation' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT98`](rules/compatibility.md#japiext98) | 'opennlp.tools.cmdline' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT103`](rules/compatibility.md#japiext103) | 'ice.scripters.js' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT109`](rules/compatibility.md#japiext109) | 'org.bouncycastle.i18n' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT111`](rules/compatibility.md#japiext111) | 'org.bouncycastle.operator' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT112`](rules/compatibility.md#japiext112) | 'schemaorg_apache_xmlbeans.system.sXMLSCHEMA' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT114`](rules/compatibility.md#japiext114) | 'opennlp.tools.chunker' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT115`](rules/compatibility.md#japiext115) | 'org.jsoup.safety' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT116`](rules/compatibility.md#japiext116) | 'org.bouncycastle.cert' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT118`](rules/compatibility.md#japiext118) | 'org.jmol.shapespecial' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT120`](rules/compatibility.md#japiext120) | 'thredds.catalog2.xml' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT121`](rules/compatibility.md#japiext121) | 'thredds.catalog.dl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT123`](rules/compatibility.md#japiext123) | 'net.jcip.annotations' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT126`](rules/compatibility.md#japiext126) | 'org.jmol.adapter' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT134`](rules/compatibility.md#japiext134) | 'opennlp.tools.formats' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT135`](rules/compatibility.md#japiext135) | 'com.mchange.v1' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT136`](rules/compatibility.md#japiext136) | 'com.lowagie.tools' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT138`](rules/compatibility.md#japiext138) | 'org.dom4j.io' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT145`](rules/compatibility.md#japiext145) | 'org.apache.jempbox' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT160`](rules/compatibility.md#japiext160) | 'ice.net.socks' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT169`](rules/compatibility.md#japiext169) | 'org.bouncycastle.x509' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT170`](rules/compatibility.md#japiext170) | 'org.jsoup.examples' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT173`](rules/compatibility.md#japiext173) | 'org.apache.mina' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT178`](rules/compatibility.md#japiext178) | 'com.mchange.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT181`](rules/compatibility.md#japiext181) | 'thredds.catalog2.builder' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT187`](rules/compatibility.md#japiext187) | 'com.drew.tools' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT192`](rules/compatibility.md#japiext192) | 'ice.util.security' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT195`](rules/compatibility.md#japiext195) | 'org.apache.james' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT196`](rules/compatibility.md#japiext196) | 'org.jmol.export' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT197`](rules/compatibility.md#japiext197) | 'org.jmol.symmetry' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT198`](rules/compatibility.md#japiext198) | 'org.mozilla.universalchardet' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT206`](rules/compatibility.md#japiext206) | 'opennlp.tools.coref' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT207`](rules/compatibility.md#japiext207) | 'opennlp.tools.postag' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT209`](rules/compatibility.md#japiext209) | 'org.mozilla.javascript' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT211`](rules/compatibility.md#japiext211) | 'ice.dom.css' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT212`](rules/compatibility.md#japiext212) | 'org.mozilla.classfile' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT218`](rules/compatibility.md#japiext218) | 'org.dom4j.datatype' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT219`](rules/compatibility.md#japiext219) | 'ice.util.unit' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT230`](rules/compatibility.md#japiext230) | 'com.drew.imaging' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT231`](rules/compatibility.md#japiext231) | 'jj2000.j2k.codestream' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT232`](rules/compatibility.md#japiext232) | 'org.jmol.popup' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT235`](rules/compatibility.md#japiext235) | 'org.apache.tika' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT240`](rules/compatibility.md#japiext240) | 'opennlp.tools.tokenize' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT242`](rules/compatibility.md#japiext242) | 'javolution.util.stripped' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT259`](rules/compatibility.md#japiext259) | 'jj2000.j2k.encoder' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT266`](rules/compatibility.md#japiext266) | 'org.bouncycastle.tsp' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT267`](rules/compatibility.md#japiext267) | 'com.cybozu.labs' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT269`](rules/compatibility.md#japiext269) | 'org.jmol.smiles' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT272`](rules/compatibility.md#japiext272) | 'com.sparshui.inputdevice' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT274`](rules/compatibility.md#japiext274) | 'ice.storm.print' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT275`](rules/compatibility.md#japiext275) | 'org.jmol.api' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT277`](rules/compatibility.md#japiext277) | 'org.dom4j.jaxb' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT281`](rules/compatibility.md#japiext281) | 'org.jmol.console' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT294`](rules/compatibility.md#japiext294) | 'org.apache.ftpserver' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT301`](rules/compatibility.md#japiext301) | 'thredds.catalog2.simpleImpl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT305`](rules/compatibility.md#japiext305) | 'org.jmol.atomdata' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT308`](rules/compatibility.md#japiext308) | 'com.almworks.sqlite4java' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT309`](rules/compatibility.md#japiext309) | 'org.bouncycastle.crypto' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT312`](rules/compatibility.md#japiext312) | 'thredds.catalog2.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT315`](rules/compatibility.md#japiext315) | 'com.rometools.utils' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT316`](rules/compatibility.md#japiext316) | 'ice.util.encoding' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT317`](rules/compatibility.md#japiext317) | 'com.mchange.lang' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT320`](rules/compatibility.md#japiext320) | 'ice.net.pac' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT321`](rules/compatibility.md#japiext321) | 'cryptix.util.core' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT324`](rules/compatibility.md#japiext324) | 'schemaorg_apache_xmlbeans.system.sXMLLANG' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT326`](rules/compatibility.md#japiext326) | 'thredds.catalog.crawl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT327`](rules/compatibility.md#japiext327) | 'thredds.catalog.query' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT330`](rules/compatibility.md#japiext330) | 'org.openscience.jmol' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT331`](rules/compatibility.md#japiext331) | 'ice.dom.html' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT336`](rules/compatibility.md#japiext336) | 'ice.pilots.applet' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT340`](rules/compatibility.md#japiext340) | 'org.jmol.modelkit' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT343`](rules/compatibility.md#japiext343) | 'jj2000.j2k.wavelet' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT348`](rules/compatibility.md#japiext348) | 'org.bouncycastle.cms' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT352`](rules/compatibility.md#japiext352) | 'org.bouncycastle.jce' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT353`](rules/compatibility.md#japiext353) | 'ice.net.mailto' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT358`](rules/compatibility.md#japiext358) | 'jj2000.j2k.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT360`](rules/compatibility.md#japiext360) | 'jj2000.j2k.quantization' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT364`](rules/compatibility.md#japiext364) | 'com.coremedia.iso' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT369`](rules/compatibility.md#japiext369) | 'thredds.cataloggen.catalogrefexpander' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT378`](rules/compatibility.md#japiext378) | 'org.jmol.script' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT380`](rules/compatibility.md#japiext380) | 'com.optimaize.langdetect' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT381`](rules/compatibility.md#japiext381) | 'net.arnx.jsonic' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT390`](rules/compatibility.md#japiext390) | 'schemaorg_apache_xmlbeans.system.sF1327CCA741569E70F9CA8C9AF9B44B2' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT394`](rules/compatibility.md#japiext394) | 'xjava.security.interfaces' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT398`](rules/compatibility.md#japiext398) | 'org.dom4j.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT401`](rules/compatibility.md#japiext401) | 'org.bouncycastle.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT403`](rules/compatibility.md#japiext403) | 'org.bouncycastle.pkcs' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT405`](rules/compatibility.md#japiext405) | 'org.bouncycastle.dvcs' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT408`](rules/compatibility.md#japiext408) | 'se.fishtank.css' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT409`](rules/compatibility.md#japiext409) | 'ice.net.doc' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT411`](rules/compatibility.md#japiext411) | 'com.adobe.xmp' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT416`](rules/compatibility.md#japiext416) | 'opennlp.tools.namefind' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT420`](rules/compatibility.md#japiext420) | 'opennlp.tools.doccat' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT421`](rules/compatibility.md#japiext421) | 'com.sun.java' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT426`](rules/compatibility.md#japiext426) | 'thredds.cataloggen.config' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT428`](rules/compatibility.md#japiext428) | 'org.bouncycastle.mozilla' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT430`](rules/compatibility.md#japiext430) | 'opennlp.tools.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT442`](rules/compatibility.md#japiext442) | 'jj2000.j2k.roi' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT445`](rules/compatibility.md#japiext445) | 'org.bouncycastle.math' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT447`](rules/compatibility.md#japiext447) | 'org.dom4j.dom' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT449`](rules/compatibility.md#japiext449) | 'jj2000.j2k.entropy' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT451`](rules/compatibility.md#japiext451) | 'org.bouncycastle.eac' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT455`](rules/compatibility.md#japiext455) | 'org.jmol.i18n' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT459`](rules/compatibility.md#japiext459) | 'thredds.inventory.filter' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT466`](rules/compatibility.md#japiext466) | 'opennlp.maxent.io' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT467`](rules/compatibility.md#japiext467) | 'net.didion.jwnl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT470`](rules/compatibility.md#japiext470) | 'cryptix.provider.rsa' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT474`](rules/compatibility.md#japiext474) | 'org.jmol.shapebio' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT477`](rules/compatibility.md#japiext477) | 'ice.pilots.image' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT485`](rules/compatibility.md#japiext485) | 'jj2000.j2k.fileformat' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT486`](rules/compatibility.md#japiext486) | 'org.bouncycastle.mail' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT487`](rules/compatibility.md#japiext487) | 'opennlp.tools.lang' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT491`](rules/compatibility.md#japiext491) | 'org.jmol.g3d' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT495`](rules/compatibility.md#japiext495) | 'cryptix.provider.cipher' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT502`](rules/compatibility.md#japiext502) | 'ice.util.net' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT503`](rules/compatibility.md#japiext503) | 'jj2000.j2k.image' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT505`](rules/compatibility.md#japiext505) | 'ice.util.io' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT508`](rules/compatibility.md#japiext508) | 'org.jmol.modelset' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT510`](rules/compatibility.md#japiext510) | 'com.mchange.v2' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT511`](rules/compatibility.md#japiext511) | 'org.dom4j.rule' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT513`](rules/compatibility.md#japiext513) | 'org.bouncycastle.pqc' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT516`](rules/compatibility.md#japiext516) | 'org.jmol.modelsetbio' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT517`](rules/compatibility.md#japiext517) | 'be.frma.langguess' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT520`](rules/compatibility.md#japiext520) | 'schemaorg_apache_xmlbeans.system.sXMLTOOLS' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT521`](rules/compatibility.md#japiext521) | 'cryptix.provider.key' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT523`](rules/compatibility.md#japiext523) | 'thredds.crawlabledataset.filter' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT533`](rules/compatibility.md#japiext533) | 'thredds.cataloggen.inserter' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT534`](rules/compatibility.md#japiext534) | 'org.codehaus.stax2' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT535`](rules/compatibility.md#japiext535) | 'jj2000.j2k.decoder' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT536`](rules/compatibility.md#japiext536) | 'org.dom4j.bean' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT540`](rules/compatibility.md#japiext540) | 'org.bouncycastle.openssl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT547`](rules/compatibility.md#japiext547) | 'ice.net.proxy' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT548`](rules/compatibility.md#japiext548) | 'org.dom4j.tree' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT553`](rules/compatibility.md#japiext553) | 'com.lowagie.bc' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT557`](rules/compatibility.md#japiext557) | 'uk.ac.rdg' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT563`](rules/compatibility.md#japiext563) | 'org.apache.sis' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT564`](rules/compatibility.md#japiext564) | 'org.dom4j.xpp' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT571`](rules/compatibility.md#japiext571) | 'com.sparshui.common' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT589`](rules/compatibility.md#japiext589) | 'Acme.JPM.Encoders' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT590`](rules/compatibility.md#japiext590) | 'org.json.zip' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT592`](rules/compatibility.md#japiext592) | 'org.bouncycastle.jcajce' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT595`](rules/compatibility.md#japiext595) | 'com.sparshui.server' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT596`](rules/compatibility.md#japiext596) | 'org.jmol.shapesurface' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT597`](rules/compatibility.md#japiext597) | 'org.bouncycastle.asn1' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT598`](rules/compatibility.md#japiext598) | 'opennlp.tools.sentdetect' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT601`](rules/compatibility.md#japiext601) | 'com.rometools.rome' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT603`](rules/compatibility.md#japiext603) | 'com.drew.lang' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT604`](rules/compatibility.md#japiext604) | 'thredds.crawlabledataset.sorter' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT605`](rules/compatibility.md#japiext605) | 'ice.util.image' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT610`](rules/compatibility.md#japiext610) | 'com.sparshui.client' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT611`](rules/compatibility.md#japiext611) | 'thredds.catalog.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT614`](rules/compatibility.md#japiext614) | 'org.bouncycastle.voms' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT616`](rules/compatibility.md#japiext616) | 'com.lowagie.text' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT618`](rules/compatibility.md#japiext618) | 'org.jsoup.helper' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT622`](rules/compatibility.md#japiext622) | 'org.jmol.shape' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT627`](rules/compatibility.md#japiext627) | 'ice.util.swing' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT628`](rules/compatibility.md#japiext628) | 'org.jmol.jvxl' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT635`](rules/compatibility.md#japiext635) | 'org.cometd.client' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT637`](rules/compatibility.md#japiext637) | 'ice.pilots.pdfgo' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT638`](rules/compatibility.md#japiext638) | 'org.json.simple' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT639`](rules/compatibility.md#japiext639) | 'org.jsoup.nodes' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT643`](rules/compatibility.md#japiext643) | 'ice.pilots.svg' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT644`](rules/compatibility.md#japiext644) | 'thredds.catalog.parser' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT647`](rules/compatibility.md#japiext647) | 'org.ccil.cowan' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT653`](rules/compatibility.md#japiext653) | 'org.jmol.geodesic' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT655`](rules/compatibility.md#japiext655) | 'jj2000.j2k.io' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT661`](rules/compatibility.md#japiext661) | 'org.dom4j.dtd' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT670`](rules/compatibility.md#japiext670) | 'cryptix.provider.md' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT671`](rules/compatibility.md#japiext671) | 'opennlp.tools.parser' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT672`](rules/compatibility.md#japiext672) | 'com.sparshui.gestures' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT680`](rules/compatibility.md#japiext680) | 'org.jmol.viewer' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT683`](rules/compatibility.md#japiext683) | 'ice.pilots.html4' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT685`](rules/compatibility.md#japiext685) | 'schemaorg_apache_xmlbeans.system.sXMLCONFIG' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT686`](rules/compatibility.md#japiext686) | 'opennlp.maxent.quasinewton' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT688`](rules/compatibility.md#japiext688) | 'org.jsoup.parser' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT689`](rules/compatibility.md#japiext689) | 'com.ctc.wstx' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT696`](rules/compatibility.md#japiext696) | 'org.jmol.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT697`](rules/compatibility.md#japiext697) | 'org.itadaki.bzip2' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT699`](rules/compatibility.md#japiext699) | 'com.codahale.metrics' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT700`](rules/compatibility.md#japiext700) | 'com.datastax.driver' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT713`](rules/compatibility.md#japiext713) | 'com.terracotta.entity' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT716`](rules/compatibility.md#japiext716) | 'io.netty.bootstrap' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT717`](rules/compatibility.md#japiext717) | 'io.netty.buffer' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT718`](rules/compatibility.md#japiext718) | 'io.netty.channel' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT719`](rules/compatibility.md#japiext719) | 'io.netty.handler' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT720`](rules/compatibility.md#japiext720) | 'io.netty.util' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT735`](rules/compatibility.md#japiext735) | 'net.sf.ehcache' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT736`](rules/compatibility.md#japiext736) | 'org.apache.directory' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT745`](rules/compatibility.md#japiext745) | 'org.joda.time' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT781`](rules/compatibility.md#japiext781) | 'org.springframework.jms' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT787`](rules/compatibility.md#japiext787) | 'org.springframework.messaging' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT791`](rules/compatibility.md#japiext791) | 'org.springframework.oxm' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT809`](rules/compatibility.md#japiext809) | 'org.terracotta.context' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT810`](rules/compatibility.md#japiext810) | 'org.terracotta.modules' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT811`](rules/compatibility.md#japiext811) | 'org.terracotta.statistics' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT813`](rules/compatibility.md#japiext813) | 'org.xerial.snappy' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT814`](rules/compatibility.md#japiext814) | 'javax.help.event' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT815`](rules/compatibility.md#japiext815) | 'javax.help.plaf' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT816`](rules/compatibility.md#japiext816) | 'javax.help.resources' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT817`](rules/compatibility.md#japiext817) | 'javax.help.search' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT818`](rules/compatibility.md#japiext818) | 'javax.help.tagext' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT824`](rules/compatibility.md#japiext824) | 'schemaorg_apache_xmlbeans.system.sD023D6490046BA0250A839A9AD24C443' Java package and subpackages are not available in MATLAB. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`ROSDFOBJECT`](rules/compatibility.md#rosdfobject) | For improved performance and code generation workflows, specify 'DataFormat' name-value argument as 'struct'. In a future release, the default format will change to 'struct'. |
| [`ROSSCFUN`](rules/compatibility.md#rosscfun) | Add the service type as the second input argument to 'rossvcclient'. In a future release, the service type will be a required input argument. |
| [`ROSSCPKG`](rules/compatibility.md#rosscpkg) | Add the service type as the third input argument to 'ros.ServiceClient'. In a future release, the service type will be a required input argument. |
| [`ROSDFMISSING`](rules/compatibility.md#rosdfmissing) | Use name-value argument 'DataFormat' to specify the message format as the default message format will change to 'struct' in a future release. |
| [`JAPIEXT2`](rules/compatibility.md#japiext2) | 'javax.annotation.security' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT3`](rules/compatibility.md#japiext3) | 'javax.annotation.sql' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT6`](rules/compatibility.md#japiext6) | 'javax.websocket.server' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT12`](rules/compatibility.md#japiext12) | 'org.jdesktop.layout' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT13`](rules/compatibility.md#japiext13) | 'org.jdesktop.swingx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT30`](rules/compatibility.md#japiext30) | 'com.jidesoft.icons' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT45`](rules/compatibility.md#japiext45) | 'info.clearthought.layout' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT46`](rules/compatibility.md#japiext46) | 'org.antlr.misc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT48`](rules/compatibility.md#japiext48) | 'org.powermock.configuration' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT49`](rules/compatibility.md#japiext49) | 'javax.mail.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT51`](rules/compatibility.md#japiext51) | 'com.thaiopensource.validate' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT52`](rules/compatibility.md#japiext52) | 'org.opengis.webservice' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT53`](rules/compatibility.md#japiext53) | 'com.google.common' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT57`](rules/compatibility.md#japiext57) | 'freemarker.ext.jython' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT58`](rules/compatibility.md#japiext58) | 'org.h2.store' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT59`](rules/compatibility.md#japiext59) | 'com.google.protobuf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT60`](rules/compatibility.md#japiext60) | 'org.h2.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT61`](rules/compatibility.md#japiext61) | 'org.jaxen.expr' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT68`](rules/compatibility.md#japiext68) | 'org.antlr.codegen' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT69`](rules/compatibility.md#japiext69) | 'org.mockito.listeners' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT70`](rules/compatibility.md#japiext70) | 'org.h2.result' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT71`](rules/compatibility.md#japiext71) | 'com.jogamp.newt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT72`](rules/compatibility.md#japiext72) | 'org.mockito.mock' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT73`](rules/compatibility.md#japiext73) | 'org.opengis.filter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT76`](rules/compatibility.md#japiext76) | 'org.h2.constraint' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT77`](rules/compatibility.md#japiext77) | 'org.powermock.tests' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT78`](rules/compatibility.md#japiext78) | 'org.h2.tools' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT79`](rules/compatibility.md#japiext79) | 'javassist.bytecode.annotation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT81`](rules/compatibility.md#japiext81) | 'org.h2.jdbc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT83`](rules/compatibility.md#japiext83) | 'org.mockito.quality' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT86`](rules/compatibility.md#japiext86) | 'javax.servlet.http' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT87`](rules/compatibility.md#japiext87) | 'org.geotools.event' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT89`](rules/compatibility.md#japiext89) | 'org.mockito.hamcrest' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT93`](rules/compatibility.md#japiext93) | 'javax.mail.event' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT94`](rules/compatibility.md#japiext94) | 'com.jgoodies.looks' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT95`](rules/compatibility.md#japiext95) | 'com.graphbuilder.math' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT99`](rules/compatibility.md#japiext99) | 'mwhtmlguitest.org.apache' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT100`](rules/compatibility.md#japiext100) | 'javax.mail.search' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT101`](rules/compatibility.md#japiext101) | 'net.sf.cglib' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT102`](rules/compatibility.md#japiext102) | 'org.powermock.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT104`](rules/compatibility.md#japiext104) | 'org.geotools.resources' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT105`](rules/compatibility.md#japiext105) | 'org.openxml4j.samples' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT106`](rules/compatibility.md#japiext106) | 'org.w3c.css' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT107`](rules/compatibility.md#japiext107) | 'org.cef.handler' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT108`](rules/compatibility.md#japiext108) | 'org.mortbay.jetty' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT110`](rules/compatibility.md#japiext110) | 'jogamp.nativewindow.awt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT113`](rules/compatibility.md#japiext113) | 'com.jidesoft.popup' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT117`](rules/compatibility.md#japiext117) | 'org.apache.batik' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT119`](rules/compatibility.md#japiext119) | 'net.jpountz.xxhash' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT122`](rules/compatibility.md#japiext122) | 'org.apache.axis2' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT124`](rules/compatibility.md#japiext124) | 'com.jogamp.opengl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT125`](rules/compatibility.md#japiext125) | 'com.reuters.sdist' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT127`](rules/compatibility.md#japiext127) | 'org.jaxen.dom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT128`](rules/compatibility.md#japiext128) | 'org.aopalliance.intercept' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT129`](rules/compatibility.md#japiext129) | 'org.jaxen.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT130`](rules/compatibility.md#japiext130) | 'jogamp.opengl.gl4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT131`](rules/compatibility.md#japiext131) | 'com.thaiopensource.datatype' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT132`](rules/compatibility.md#japiext132) | 'com.jidesoft.awt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT133`](rules/compatibility.md#japiext133) | 'com.graphbuilder.curve' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT137`](rules/compatibility.md#japiext137) | 'org.jacoco.ant' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT139`](rules/compatibility.md#japiext139) | 'org.objenesis.instantiator' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT140`](rules/compatibility.md#japiext140) | 'net.jini.event' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT141`](rules/compatibility.md#japiext141) | 'edu.uci.ics' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT142`](rules/compatibility.md#japiext142) | 'org.geotools.parameter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT143`](rules/compatibility.md#japiext143) | 'com.googlecode.javaewah32' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT144`](rules/compatibility.md#japiext144) | 'org.jaxen.function' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT146`](rules/compatibility.md#japiext146) | 'com.jidesoft.spinner' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT147`](rules/compatibility.md#japiext147) | 'org.mockito.exceptions' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT148`](rules/compatibility.md#japiext148) | 'org.objectweb.asm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT149`](rules/compatibility.md#japiext149) | 'org.jdom2.located' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT150`](rules/compatibility.md#japiext150) | 'org.mortbay.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT151`](rules/compatibility.md#japiext151) | 'antlr.debug.misc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT152`](rules/compatibility.md#japiext152) | 'javax.servlet.annotation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT153`](rules/compatibility.md#japiext153) | 'net.jini.space' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT154`](rules/compatibility.md#japiext154) | 'org.apache.http' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT155`](rules/compatibility.md#japiext155) | 'org.jdom2.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT156`](rules/compatibility.md#japiext156) | 'org.antlr.stringtemplate' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT157`](rules/compatibility.md#japiext157) | 'com.graphbuilder.geom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT158`](rules/compatibility.md#japiext158) | 'antlr.actions.python' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT159`](rules/compatibility.md#japiext159) | 'org.eclipse.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT161`](rules/compatibility.md#japiext161) | 'org.w3c.dom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT162`](rules/compatibility.md#japiext162) | 'javassist.tools.reflect' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT163`](rules/compatibility.md#japiext163) | 'com.jidesoft.range' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT164`](rules/compatibility.md#japiext164) | 'org.jdom.transform' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT165`](rules/compatibility.md#japiext165) | 'org.mockito.stubbing' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT166`](rules/compatibility.md#japiext166) | 'org.iso_relax.ant' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT167`](rules/compatibility.md#japiext167) | 'org.eclipse.xtend2' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT168`](rules/compatibility.md#japiext168) | 'jogamp.newt.event' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT171`](rules/compatibility.md#japiext171) | 'jogamp.graph.font' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT172`](rules/compatibility.md#japiext172) | 'org.jdom2.xpath' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT174`](rules/compatibility.md#japiext174) | 'org.jaxen.xom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT175`](rules/compatibility.md#japiext175) | 'org.geotools.factory' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT176`](rules/compatibility.md#japiext176) | 'javax.xml.datatype' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT177`](rules/compatibility.md#japiext177) | 'net.jini.lookup' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT179`](rules/compatibility.md#japiext179) | 'org.eclipse.osgi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT180`](rules/compatibility.md#japiext180) | 'abbot.editor.recorder' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT182`](rules/compatibility.md#japiext182) | 'org.h2.mvstore' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT183`](rules/compatibility.md#japiext183) | 'org.mortbay.resource' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT184`](rules/compatibility.md#japiext184) | 'jogamp.opengl.awt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT185`](rules/compatibility.md#japiext185) | 'jogamp.opengl.x11' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT186`](rules/compatibility.md#japiext186) | 'org.tanukisoftware.wrapper' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT188`](rules/compatibility.md#japiext188) | 'org.geotools.map' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT189`](rules/compatibility.md#japiext189) | 'org.cyberneko.html' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT190`](rules/compatibility.md#japiext190) | 'org.openxmlformats.schemas' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT191`](rules/compatibility.md#japiext191) | 'com.reuters.rmtes' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT193`](rules/compatibility.md#japiext193) | 'net.bytebuddy.utility' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT194`](rules/compatibility.md#japiext194) | 'freemarker.ext.jsp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT199`](rules/compatibility.md#japiext199) | 'org.mockito.plugins' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT200`](rules/compatibility.md#japiext200) | 'org.mockito.verification' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT201`](rules/compatibility.md#japiext201) | 'com.sun.common' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT202`](rules/compatibility.md#japiext202) | 'jogamp.common.jvm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT203`](rules/compatibility.md#japiext203) | 'org.geotools.gml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT204`](rules/compatibility.md#japiext204) | 'org.hamcrest.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT205`](rules/compatibility.md#japiext205) | 'freemarker.ext.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT208`](rules/compatibility.md#japiext208) | 'net.jini.discovery' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT210`](rules/compatibility.md#japiext210) | 'org.h2.jmx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT213`](rules/compatibility.md#japiext213) | 'net.jini.io' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT214`](rules/compatibility.md#japiext214) | 'antlr.actions.csharp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT215`](rules/compatibility.md#japiext215) | 'javassist.bytecode.analysis' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT216`](rules/compatibility.md#japiext216) | 'javax.xml.transform' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT217`](rules/compatibility.md#japiext217) | 'abbot.editor.actions' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT220`](rules/compatibility.md#japiext220) | 'org.jacoco.report' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT221`](rules/compatibility.md#japiext221) | 'org.powermock.reflect' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT222`](rules/compatibility.md#japiext222) | 'jogamp.opengl.macosx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT223`](rules/compatibility.md#japiext223) | 'org.apache.xmlcommons' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT224`](rules/compatibility.md#japiext224) | 'org.jaxen.javabean' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT225`](rules/compatibility.md#japiext225) | 'net.sf.xslthl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT226`](rules/compatibility.md#japiext226) | 'org.mockito.codegen' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT227`](rules/compatibility.md#japiext227) | 'net.jini.activation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT228`](rules/compatibility.md#japiext228) | 'net.bytebuddy.matcher' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT229`](rules/compatibility.md#japiext229) | 'net.bytebuddy.dynamic' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT233`](rules/compatibility.md#japiext233) | 'org.mockito.invocation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT234`](rules/compatibility.md#japiext234) | 'org.hamcrest.number' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT236`](rules/compatibility.md#japiext236) | 'org.openxml4j.document' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT237`](rules/compatibility.md#japiext237) | 'freemarker.ext.jdom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT238`](rules/compatibility.md#japiext238) | 'org.geotools.feature' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT239`](rules/compatibility.md#japiext239) | 'org.eclipse.jgit' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT241`](rules/compatibility.md#japiext241) | 'org.h2.server' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT243`](rules/compatibility.md#japiext243) | 'net.jini.security' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT244`](rules/compatibility.md#japiext244) | 'com.jidesoft.csv' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT245`](rules/compatibility.md#japiext245) | 'org.tmatesoft.svn' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT246`](rules/compatibility.md#japiext246) | 'freemarker.ext.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT247`](rules/compatibility.md#japiext247) | 'org.antlr.grammar' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT248`](rules/compatibility.md#japiext248) | 'com.jidesoft.field' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT249`](rules/compatibility.md#japiext249) | 'jogamp.nativewindow.windows' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT250`](rules/compatibility.md#japiext250) | 'jogamp.nativewindow.macosx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT251`](rules/compatibility.md#japiext251) | 'org.geotools.coverage' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT252`](rules/compatibility.md#japiext252) | 'org.h2.bnf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT253`](rules/compatibility.md#japiext253) | 'org.h2.jdbcx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT254`](rules/compatibility.md#japiext254) | 'org.mockito.session' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT255`](rules/compatibility.md#japiext255) | 'org.powermock.mockpolicies' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT256`](rules/compatibility.md#japiext256) | 'jogamp.common.os' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT257`](rules/compatibility.md#japiext257) | 'org.apache.fontbox' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT258`](rules/compatibility.md#japiext258) | 'net.jini.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT260`](rules/compatibility.md#japiext260) | 'org.apache.taglibs' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT261`](rules/compatibility.md#japiext261) | 'org.jacoco.agent' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT262`](rules/compatibility.md#japiext262) | 'freemarker.ext.servlet' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT263`](rules/compatibility.md#japiext263) | 'jogamp.opengl.gl2' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT264`](rules/compatibility.md#japiext264) | 'com.thaiopensource.relaxng' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT265`](rules/compatibility.md#japiext265) | 'org.h2.security' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT268`](rules/compatibility.md#japiext268) | 'org.geotools.measure' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT270`](rules/compatibility.md#japiext270) | 'org.cometd.websocket' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT271`](rules/compatibility.md#japiext271) | 'org.jdom2.input' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT273`](rules/compatibility.md#japiext273) | 'abbot.editor.editors' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT276`](rules/compatibility.md#japiext276) | 'org.eclipse.xtext' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT278`](rules/compatibility.md#japiext278) | 'org.etsi.uri' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT279`](rules/compatibility.md#japiext279) | 'org.opengis.spatialschema' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT280`](rules/compatibility.md#japiext280) | 'org.opengis.feature' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT282`](rules/compatibility.md#japiext282) | 'com.vividsolutions.jts' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT283`](rules/compatibility.md#japiext283) | 'org.apache.ws' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT284`](rules/compatibility.md#japiext284) | 'com.intel.bluetooth' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT285`](rules/compatibility.md#japiext285) | 'com.jidesoft.alert' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT286`](rules/compatibility.md#japiext286) | 'org.iso_relax.dispatcher' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT287`](rules/compatibility.md#japiext287) | 'org.antlr.gunit' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT288`](rules/compatibility.md#japiext288) | 'jogamp.newt.swt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT289`](rules/compatibility.md#japiext289) | 'com.jidesoft.margin' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT290`](rules/compatibility.md#japiext290) | 'org.geotools.data' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT291`](rules/compatibility.md#japiext291) | 'org.cef.browser' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT292`](rules/compatibility.md#japiext292) | 'com.jogamp.graph' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT293`](rules/compatibility.md#japiext293) | 'org.geotools.io' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT295`](rules/compatibility.md#japiext295) | 'org.apache.jasper' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT296`](rules/compatibility.md#japiext296) | 'com.thoughtworks.xstream' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT297`](rules/compatibility.md#japiext297) | 'org.apache.commons' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT298`](rules/compatibility.md#japiext298) | 'org.h2.command' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT299`](rules/compatibility.md#japiext299) | 'org.geotools.geometry' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT300`](rules/compatibility.md#japiext300) | 'com.vladium.jcd' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT302`](rules/compatibility.md#japiext302) | 'org.junit.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT303`](rules/compatibility.md#japiext303) | 'org.powermock.api' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT304`](rules/compatibility.md#japiext304) | 'net.sf.saxon' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT306`](rules/compatibility.md#japiext306) | 'com.bloomberglp.blpapi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT307`](rules/compatibility.md#japiext307) | 'com.reuters.io' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT310`](rules/compatibility.md#japiext310) | 'freemarker.debug.impl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT311`](rules/compatibility.md#japiext311) | 'javax.servlet.descriptor' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT313`](rules/compatibility.md#japiext313) | 'com.sun.midp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT314`](rules/compatibility.md#japiext314) | 'com.jidesoft.utils' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT318`](rules/compatibility.md#japiext318) | 'javax.mail.internet' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT319`](rules/compatibility.md#japiext319) | 'abbot.script.parsers' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT322`](rules/compatibility.md#japiext322) | 'com.vladium.logging' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT323`](rules/compatibility.md#japiext323) | 'freemarker.ext.ant' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT325`](rules/compatibility.md#japiext325) | 'org.aopalliance.aop' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT328`](rules/compatibility.md#japiext328) | 'org.h2.message' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT329`](rules/compatibility.md#japiext329) | 'com.jogamp.common' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT332`](rules/compatibility.md#japiext332) | 'javax.xml.validation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT333`](rules/compatibility.md#japiext333) | 'org.eclipse.jdt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT334`](rules/compatibility.md#japiext334) | 'org.mortbay.start' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT335`](rules/compatibility.md#japiext335) | 'javax.wsdl.extensions' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT337`](rules/compatibility.md#japiext337) | 'org.opengis.metadata' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT338`](rules/compatibility.md#japiext338) | 'org.slf4j.impl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT339`](rules/compatibility.md#japiext339) | 'org.h2.index' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT341`](rules/compatibility.md#japiext341) | 'com.jidesoft.jdk' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT342`](rules/compatibility.md#japiext342) | 'com.jidesoft.navigation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT344`](rules/compatibility.md#japiext344) | 'freemarker.ext.beans' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT345`](rules/compatibility.md#japiext345) | 'org.eclipse.e4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT346`](rules/compatibility.md#japiext346) | 'org.junit.runner' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT347`](rules/compatibility.md#japiext347) | 'org.apache.xmlgraphics' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT349`](rules/compatibility.md#japiext349) | 'com.jidesoft.pane' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT350`](rules/compatibility.md#japiext350) | 'org.hamcrest.generator' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT351`](rules/compatibility.md#japiext351) | 'org.iso_relax.verifier' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT354`](rules/compatibility.md#japiext354) | 'org.opengis.coverage' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT355`](rules/compatibility.md#japiext355) | 'org.antlr.tool' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT356`](rules/compatibility.md#japiext356) | 'org.mortbay.thread' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT357`](rules/compatibility.md#japiext357) | 'org.mortbay.naming' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT359`](rules/compatibility.md#japiext359) | 'com.jidesoft.grouper' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT361`](rules/compatibility.md#japiext361) | 'jogamp.opengl.windows' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT362`](rules/compatibility.md#japiext362) | 'org.tmatesoft.sqljet' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT363`](rules/compatibility.md#japiext363) | 'org.geotools.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT365`](rules/compatibility.md#japiext365) | 'org.cef.network' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT366`](rules/compatibility.md#japiext366) | 'antlr.actions.cpp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT367`](rules/compatibility.md#japiext367) | 'org.w3.x2000' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT368`](rules/compatibility.md#japiext368) | 'net.bytebuddy.description' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT370`](rules/compatibility.md#japiext370) | 'org.cometd.server' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT371`](rules/compatibility.md#japiext371) | 'org.h2.value' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT372`](rules/compatibility.md#japiext372) | 'org.opengis.referencing' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT373`](rules/compatibility.md#japiext373) | 'org.antlr.analysis' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT374`](rules/compatibility.md#japiext374) | 'org.openxml4j.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT375`](rules/compatibility.md#japiext375) | 'com.vladium.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT376`](rules/compatibility.md#japiext376) | 'net.jini.loader' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT377`](rules/compatibility.md#japiext377) | 'org.apache.axiom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT379`](rules/compatibility.md#japiext379) | 'org.hamcrest.integration' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT382`](rules/compatibility.md#japiext382) | 'com.jogamp.gluegen' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT383`](rules/compatibility.md#japiext383) | 'com.reuters.sticapi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT384`](rules/compatibility.md#japiext384) | 'com.reuters.ansi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT385`](rules/compatibility.md#japiext385) | 'org.opengis.layer' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT386`](rules/compatibility.md#japiext386) | 'org.jdom.input' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT387`](rules/compatibility.md#japiext387) | 'com.jidesoft.wizard' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT388`](rules/compatibility.md#japiext388) | 'org.easymock.cglib' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT389`](rules/compatibility.md#japiext389) | 'javax.microedition.io' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT391`](rules/compatibility.md#japiext391) | 'net.jini.jeri' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT392`](rules/compatibility.md#japiext392) | 'com.graphbuilder.struc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT393`](rules/compatibility.md#japiext393) | 'net.jini.id' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT395`](rules/compatibility.md#japiext395) | 'com.sun.enterprise' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT396`](rules/compatibility.md#japiext396) | 'javassist.tools.web' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT397`](rules/compatibility.md#japiext397) | 'org.cometd.bayeux' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT399`](rules/compatibility.md#japiext399) | 'jogamp.newt.driver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT400`](rules/compatibility.md#japiext400) | 'javax.xml.xquery' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT402`](rules/compatibility.md#japiext402) | 'org.jdom.xpath' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT404`](rules/compatibility.md#japiext404) | 'org.xml.sax' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT406`](rules/compatibility.md#japiext406) | 'junit.extensions.abbot' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT407`](rules/compatibility.md#japiext407) | 'org.junit.matchers' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT410`](rules/compatibility.md#japiext410) | 'org.osgi.service' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT412`](rules/compatibility.md#japiext412) | 'org.mockito.configuration' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT413`](rules/compatibility.md#japiext413) | 'org.eclipse.ui' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT414`](rules/compatibility.md#japiext414) | 'org.opengis.go' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT415`](rules/compatibility.md#japiext415) | 'org.opengis.sld' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT417`](rules/compatibility.md#japiext417) | 'javax.wsdl.factory' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT418`](rules/compatibility.md#japiext418) | 'jogamp.opengl.es3' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT419`](rules/compatibility.md#japiext419) | 'org.apache.wml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT422`](rules/compatibility.md#japiext422) | 'org.geotools.catalog' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT423`](rules/compatibility.md#japiext423) | 'org.mockito.runners' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT424`](rules/compatibility.md#japiext424) | 'com.ibm.oti' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT425`](rules/compatibility.md#japiext425) | 'antlr.collections.impl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT427`](rules/compatibility.md#japiext427) | 'org.slf4j.helpers' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT429`](rules/compatibility.md#japiext429) | 'org.osgi.framework' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT431`](rules/compatibility.md#japiext431) | 'org.apache.xerces' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT432`](rules/compatibility.md#japiext432) | 'com.sun.appserv' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT433`](rules/compatibility.md#japiext433) | 'org.mockito.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT434`](rules/compatibility.md#japiext434) | 'org.tartarus.snowball' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT435`](rules/compatibility.md#japiext435) | 'org.cometd.common' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT436`](rules/compatibility.md#japiext436) | 'com.trilead.ssh2' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT437`](rules/compatibility.md#japiext437) | 'org.hamcrest.beans' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT438`](rules/compatibility.md#japiext438) | 'de.regnis.q' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT439`](rules/compatibility.md#japiext439) | 'org.h2.fulltext' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT440`](rules/compatibility.md#japiext440) | 'org.h2.upgrade' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT441`](rules/compatibility.md#japiext441) | 'org.easymock.asm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT443`](rules/compatibility.md#japiext443) | 'ca.odell.glazedlists' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT444`](rules/compatibility.md#japiext444) | 'javax.xml.xpath' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT446`](rules/compatibility.md#japiext446) | 'org.apache.lucene' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT448`](rules/compatibility.md#japiext448) | 'javassist.compiler.ast' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT450`](rules/compatibility.md#japiext450) | 'com.jidesoft.hints' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT452`](rules/compatibility.md#japiext452) | 'org.h2.schema' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT453`](rules/compatibility.md#japiext453) | 'org.jdom2.adapters' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT454`](rules/compatibility.md#japiext454) | 'org.mortbay.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT456`](rules/compatibility.md#japiext456) | 'jogamp.graph.curve' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT457`](rules/compatibility.md#japiext457) | 'com.jidesoft.chart' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT458`](rules/compatibility.md#japiext458) | 'com.jidesoft.grid' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT460`](rules/compatibility.md#japiext460) | 'org.jaxen.saxpath' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT461`](rules/compatibility.md#japiext461) | 'org.slf4j.spi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT462`](rules/compatibility.md#japiext462) | 'jogamp.opengl.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT463`](rules/compatibility.md#japiext463) | 'com.jidesoft.gauge' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT464`](rules/compatibility.md#japiext464) | 'com.jgoodies.forms' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT465`](rules/compatibility.md#japiext465) | 'com.jidesoft.shortcut' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT468`](rules/compatibility.md#japiext468) | 'com.icl.saxon' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT469`](rules/compatibility.md#japiext469) | 'javassist.tools.rmi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT471`](rules/compatibility.md#japiext471) | 'org.geotools.referencing' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT472`](rules/compatibility.md#japiext472) | 'org.opengis.temporal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT473`](rules/compatibility.md#japiext473) | 'javassist.util.proxy' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT475`](rules/compatibility.md#japiext475) | 'org.mortbay.log' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT476`](rules/compatibility.md#japiext476) | 'com.google.gson' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT478`](rules/compatibility.md#japiext478) | 'org.iso_relax.catalog' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT479`](rules/compatibility.md#japiext479) | 'com.jidesoft.combobox' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT480`](rules/compatibility.md#japiext480) | 'org.opengis.parameter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT481`](rules/compatibility.md#japiext481) | 'org.geotools.metadata' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT482`](rules/compatibility.md#japiext482) | 'jogamp.common.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT483`](rules/compatibility.md#japiext483) | 'com.googlecode.javaewah' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT484`](rules/compatibility.md#japiext484) | 'org.mockito.creation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT488`](rules/compatibility.md#japiext488) | 'net.jini.config' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT489`](rules/compatibility.md#japiext489) | 'net.bytebuddy.implementation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT490`](rules/compatibility.md#japiext490) | 'org.intellij.lang' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT492`](rules/compatibility.md#japiext492) | 'org.objenesis.strategy' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT493`](rules/compatibility.md#japiext493) | 'org.eclipse.emf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT494`](rules/compatibility.md#japiext494) | 'org.cef.callback' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT496`](rules/compatibility.md#japiext496) | 'abbot.editor.widgets' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT497`](rules/compatibility.md#japiext497) | 'net.bytebuddy.jar' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT498`](rules/compatibility.md#japiext498) | 'org.eclipse.elk' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT499`](rules/compatibility.md#japiext499) | 'org.opengis.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT500`](rules/compatibility.md#japiext500) | 'org.jdom.filter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT501`](rules/compatibility.md#japiext501) | 'net.jini.export' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT504`](rules/compatibility.md#japiext504) | 'org.geotools.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT506`](rules/compatibility.md#japiext506) | 'org.h2.expression' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT507`](rules/compatibility.md#japiext507) | 'org.mortbay.servlet' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT509`](rules/compatibility.md#japiext509) | 'org.apache.log4j' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT512`](rules/compatibility.md#japiext512) | 'freemarker.ext.rhino' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT514`](rules/compatibility.md#japiext514) | 'jogamp.newt.awt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT515`](rules/compatibility.md#japiext515) | 'com.google.inject' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT518`](rules/compatibility.md#japiext518) | 'com.jidesoft.dialog' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT519`](rules/compatibility.md#japiext519) | 'net.bytebuddy.build' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT522`](rules/compatibility.md#japiext522) | 'org.apache.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT524`](rules/compatibility.md#japiext524) | 'org.antlr.runtime' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT525`](rules/compatibility.md#japiext525) | 'jogamp.opengl.egl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT526`](rules/compatibility.md#japiext526) | 'org.geotools.nature' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT527`](rules/compatibility.md#japiext527) | 'org.junit.runners' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT528`](rules/compatibility.md#japiext528) | 'com.microsoft.schemas' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT529`](rules/compatibility.md#japiext529) | 'org.mortbay.component' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT530`](rules/compatibility.md#japiext530) | 'org.apache.neethi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT531`](rules/compatibility.md#japiext531) | 'org.apache.tools' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT532`](rules/compatibility.md#japiext532) | 'com.reuters.rfa' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT537`](rules/compatibility.md#japiext537) | 'org.apache.xmpbox' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT538`](rules/compatibility.md#japiext538) | 'org.jdom.adapters' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT539`](rules/compatibility.md#japiext539) | 'net.jini.admin' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT541`](rules/compatibility.md#japiext541) | 'com.sun.jna' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT542`](rules/compatibility.md#japiext542) | 'net.jini.url' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT543`](rules/compatibility.md#japiext543) | 'org.mortbay.io' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT544`](rules/compatibility.md#japiext544) | 'org.geotools.filter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT545`](rules/compatibility.md#japiext545) | 'org.jaxen.jdom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT546`](rules/compatibility.md#japiext546) | 'com.graphbuilder.org' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT549`](rules/compatibility.md#japiext549) | 'org.h2.engine' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT550`](rules/compatibility.md#japiext550) | 'org.apache.xmlbeans' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT551`](rules/compatibility.md#japiext551) | 'com.jidesoft.tree' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT552`](rules/compatibility.md#japiext552) | 'jp.gr.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT554`](rules/compatibility.md#japiext554) | 'org.h2.table' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT555`](rules/compatibility.md#japiext555) | 'org.geotools.styling' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT556`](rules/compatibility.md#japiext556) | 'org.jdom2.filter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT558`](rules/compatibility.md#japiext558) | 'com.google.thirdparty' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT559`](rules/compatibility.md#japiext559) | 'com.jidesoft.marker' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT560`](rules/compatibility.md#japiext560) | 'org.junit.experimental' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT561`](rules/compatibility.md#japiext561) | 'com.sun.el' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT562`](rules/compatibility.md#japiext562) | 'org.jdom2.output' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT565`](rules/compatibility.md#japiext565) | 'org.geotools.image' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT566`](rules/compatibility.md#japiext566) | 'org.jdom.output' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT567`](rules/compatibility.md#japiext567) | 'org.hamcrest.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT568`](rules/compatibility.md#japiext568) | 'javassist.bytecode.stackmap' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT569`](rules/compatibility.md#japiext569) | 'javax.wsdl.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT570`](rules/compatibility.md#japiext570) | 'jogamp.graph.geom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT572`](rules/compatibility.md#japiext572) | 'com.thaiopensource.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT573`](rules/compatibility.md#japiext573) | 'org.openxml4j.exceptions' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT574`](rules/compatibility.md#japiext574) | 'jogamp.nativewindow.jawt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT575`](rules/compatibility.md#japiext575) | 'org.h2.compress' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT576`](rules/compatibility.md#japiext576) | 'net.jini.constraint' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT577`](rules/compatibility.md#japiext577) | 'org.w3c.xsl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT578`](rules/compatibility.md#japiext578) | 'net.jini.iiop' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT579`](rules/compatibility.md#japiext579) | 'com.ibm.wsdl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT580`](rules/compatibility.md#japiext580) | 'org.jetbrains.annotations' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT581`](rules/compatibility.md#japiext581) | 'org.geotools.math' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT582`](rules/compatibility.md#japiext582) | 'com.jidesoft.status' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT583`](rules/compatibility.md#japiext583) | 'org.easymock.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT584`](rules/compatibility.md#japiext584) | 'com.jidesoft.swing' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT585`](rules/compatibility.md#japiext585) | 'com.silveregg.wrapper' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT586`](rules/compatibility.md#japiext586) | 'org.jacoco.asm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT587`](rules/compatibility.md#japiext587) | 'org.apache.pdfbox' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT588`](rules/compatibility.md#japiext588) | 'jogamp.opengl.glu' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT591`](rules/compatibility.md#japiext591) | 'com.jidesoft.action' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT593`](rules/compatibility.md#japiext593) | 'com.reuters.ts1' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT594`](rules/compatibility.md#japiext594) | 'org.jaxen.pattern' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT599`](rules/compatibility.md#japiext599) | 'org.jaxen.dom4j' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT600`](rules/compatibility.md#japiext600) | 'net.jpountz.lz4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT602`](rules/compatibility.md#japiext602) | 'org.eclipse.jetty' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT606`](rules/compatibility.md#japiext606) | 'net.bytebuddy.asm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT607`](rules/compatibility.md#japiext607) | 'org.jacoco.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT608`](rules/compatibility.md#japiext608) | 'com.jcraft.jsch' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT609`](rules/compatibility.md#japiext609) | 'com.jidesoft.tooltip' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT612`](rules/compatibility.md#japiext612) | 'org.powermock.classloading' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT613`](rules/compatibility.md#japiext613) | 'org.hamcrest.text' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT615`](rules/compatibility.md#japiext615) | 'com.jidesoft.validation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT617`](rules/compatibility.md#japiext617) | 'net.jpountz.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT619`](rules/compatibility.md#japiext619) | 'org.hamcrest.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT620`](rules/compatibility.md#japiext620) | 'org.hamcrest.object' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT621`](rules/compatibility.md#japiext621) | 'org.osgi.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT623`](rules/compatibility.md#japiext623) | 'com.jidesoft.introspector' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT624`](rules/compatibility.md#japiext624) | 'org.mockito.junit' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT625`](rules/compatibility.md#japiext625) | 'com.reuters.ipc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT626`](rules/compatibility.md#japiext626) | 'com.jidesoft.lucene' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT629`](rules/compatibility.md#japiext629) | 'javax.xml.parsers' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT630`](rules/compatibility.md#japiext630) | 'org.relaxng.datatype' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT631`](rules/compatibility.md#japiext631) | 'com.fasterxml.jackson' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT632`](rules/compatibility.md#japiext632) | 'com.jidesoft.filter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT633`](rules/compatibility.md#japiext633) | 'org.cef.misc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT634`](rules/compatibility.md#japiext634) | 'com.jidesoft.docking' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT636`](rules/compatibility.md#japiext636) | 'com.sun.org' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT640`](rules/compatibility.md#japiext640) | 'net.bytebuddy.agent' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT641`](rules/compatibility.md#japiext641) | 'com.reuters.tibmsg' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT642`](rules/compatibility.md#japiext642) | 'org.jdesktop.animation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT645`](rules/compatibility.md#japiext645) | 'com.jidesoft.plaf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT646`](rules/compatibility.md#japiext646) | 'com.sun.mail' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT648`](rules/compatibility.md#japiext648) | 'org.apache.poi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT649`](rules/compatibility.md#japiext649) | 'javax.servlet.jsp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT650`](rules/compatibility.md#japiext650) | 'net.jini.jrmp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT651`](rules/compatibility.md#japiext651) | 'abbot.finder.matchers' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT652`](rules/compatibility.md#japiext652) | 'com.jidesoft.animation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT654`](rules/compatibility.md#japiext654) | 'com.jidesoft.tipoftheday' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT656`](rules/compatibility.md#japiext656) | 'com.reuters.sass3j' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT657`](rules/compatibility.md#japiext657) | 'com.reuters.mainloop' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT658`](rules/compatibility.md#japiext658) | 'com.reuters.ssl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT659`](rules/compatibility.md#japiext659) | 'org.powermock.utils' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT660`](rules/compatibility.md#japiext660) | 'com.reuters.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT662`](rules/compatibility.md#japiext662) | 'jogamp.nativewindow.x11' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT663`](rules/compatibility.md#japiext663) | 'com.vladium.emma' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT664`](rules/compatibility.md#japiext664) | 'org.junit.rules' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT665`](rules/compatibility.md#japiext665) | 'com.jidesoft.comparator' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT666`](rules/compatibility.md#japiext666) | 'com.jidesoft.document' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT667`](rules/compatibility.md#japiext667) | 'org.h2.api' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT668`](rules/compatibility.md#japiext668) | 'org.jdom2.transform' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT669`](rules/compatibility.md#japiext669) | 'antlr.actions.java' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT673`](rules/compatibility.md#japiext673) | 'com.jidesoft.list' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT674`](rules/compatibility.md#japiext674) | 'org.apache.fop' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT675`](rules/compatibility.md#japiext675) | 'freemarker.template.utility' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT676`](rules/compatibility.md#japiext676) | 'jogamp.opengl.es1' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT677`](rules/compatibility.md#japiext677) | 'org.junit.validator' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT678`](rules/compatibility.md#japiext678) | 'net.jini.lease' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT679`](rules/compatibility.md#japiext679) | 'com.jidesoft.converter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT681`](rules/compatibility.md#japiext681) | 'freemarker.ext.dom' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT682`](rules/compatibility.md#japiext682) | 'org.iso_relax.jaxp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT684`](rules/compatibility.md#japiext684) | 'com.jogamp.nativewindow' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT687`](rules/compatibility.md#japiext687) | 'com.thaiopensource.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT690`](rules/compatibility.md#japiext690) | 'org.powermock.modules' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT691`](rules/compatibility.md#japiext691) | 'org.geotools.ows' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT692`](rules/compatibility.md#japiext692) | 'net.bytebuddy.pool' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT693`](rules/compatibility.md#japiext693) | 'com.vladium.app' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT694`](rules/compatibility.md#japiext694) | 'com.jidesoft.hssf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT695`](rules/compatibility.md#japiext695) | 'com.sun.cdc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT698`](rules/compatibility.md#japiext698) | 'com.sun.activation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT701`](rules/compatibility.md#japiext701) | 'com.googlecode.concurrentlinkedhashmap' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT702`](rules/compatibility.md#japiext702) | 'com.hp.hpl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT703`](rules/compatibility.md#japiext703) | 'com.ibm.icu' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT704`](rules/compatibility.md#japiext704) | 'com.microsoft.sqlserver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT705`](rules/compatibility.md#japiext705) | 'commonj.sdo.impl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT706`](rules/compatibility.md#japiext706) | 'com.mysql.cj' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT707`](rules/compatibility.md#japiext707) | 'com.mysql.jdbc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT708`](rules/compatibility.md#japiext708) | 'com.orientechnologies.common' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT709`](rules/compatibility.md#japiext709) | 'com.orientechnologies.nio' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT710`](rules/compatibility.md#japiext710) | 'com.orientechnologies.orient' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT711`](rules/compatibility.md#japiext711) | 'com.sun.istack' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT712`](rules/compatibility.md#japiext712) | 'com.sun.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT714`](rules/compatibility.md#japiext714) | 'io.jsonwebtoken.impl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT715`](rules/compatibility.md#japiext715) | 'io.jsonwebtoken.lang' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT721`](rules/compatibility.md#japiext721) | 'javax.json.spi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT722`](rules/compatibility.md#japiext722) | 'javax.json.stream' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT723`](rules/compatibility.md#japiext723) | 'javax.persistence.criteria' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT724`](rules/compatibility.md#japiext724) | 'javax.persistence.metamodel' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT725`](rules/compatibility.md#japiext725) | 'javax.persistence.spi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT726`](rules/compatibility.md#japiext726) | 'javax.ws.rs' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT727`](rules/compatibility.md#japiext727) | 'javax.xml.bind' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT728`](rules/compatibility.md#japiext728) | 'junit.extensions.jfcunit' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT729`](rules/compatibility.md#japiext729) | 'junit.extensions.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT730`](rules/compatibility.md#japiext730) | 'mssql.googlecode.cityhash' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT731`](rules/compatibility.md#japiext731) | 'mssql.googlecode.concurrentlinkedhashmap' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT732`](rules/compatibility.md#japiext732) | 'net.oauth.client' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT733`](rules/compatibility.md#japiext733) | 'net.oauth.http' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT734`](rules/compatibility.md#japiext734) | 'net.oauth.signature' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT737`](rules/compatibility.md#japiext737) | 'org.apache.geronimo' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT738`](rules/compatibility.md#japiext738) | 'org.apache.jena' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT739`](rules/compatibility.md#japiext739) | 'org.apache.regexp' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT740`](rules/compatibility.md#japiext740) | 'org.apache.wink' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT741`](rules/compatibility.md#japiext741) | 'org.custommonkey.xmlunit' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT742`](rules/compatibility.md#japiext742) | 'org.eclipse.lyo' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT743`](rules/compatibility.md#japiext743) | 'org.eclipse.persistence' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT744`](rules/compatibility.md#japiext744) | 'org.jdesktop.jxlayer' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT746`](rules/compatibility.md#japiext746) | 'org.neo4j.driver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT747`](rules/compatibility.md#japiext747) | 'org.netbeans.jemmy' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT748`](rules/compatibility.md#japiext748) | 'org.postgresql.copy' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT749`](rules/compatibility.md#japiext749) | 'org.postgresql.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT750`](rules/compatibility.md#japiext750) | 'org.postgresql.ds' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT751`](rules/compatibility.md#japiext751) | 'org.postgresql.fastpath' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT752`](rules/compatibility.md#japiext752) | 'org.postgresql.geometric' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT753`](rules/compatibility.md#japiext753) | 'org.postgresql.gss' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT754`](rules/compatibility.md#japiext754) | 'org.postgresql.hostchooser' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT755`](rules/compatibility.md#japiext755) | 'org.postgresql.jdbc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT756`](rules/compatibility.md#japiext756) | 'org.postgresql.jdbc2' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT757`](rules/compatibility.md#japiext757) | 'org.postgresql.jdbc3' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT758`](rules/compatibility.md#japiext758) | 'org.postgresql.largeobject' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT759`](rules/compatibility.md#japiext759) | 'org.postgresql.osgi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT760`](rules/compatibility.md#japiext760) | 'org.postgresql.ssl' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT761`](rules/compatibility.md#japiext761) | 'org.postgresql.sspi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT762`](rules/compatibility.md#japiext762) | 'org.postgresql.translation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT763`](rules/compatibility.md#japiext763) | 'org.postgresql.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT764`](rules/compatibility.md#japiext764) | 'org.postgresql.xa' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT765`](rules/compatibility.md#japiext765) | 'org.springframework.aop' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT766`](rules/compatibility.md#japiext766) | 'org.springframework.asm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT767`](rules/compatibility.md#japiext767) | 'org.springframework.beans' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT768`](rules/compatibility.md#japiext768) | 'org.springframework.boot' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT769`](rules/compatibility.md#japiext769) | 'org.springframework.cache' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT770`](rules/compatibility.md#japiext770) | 'org.springframework.cglib' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT771`](rules/compatibility.md#japiext771) | 'org.springframework.context' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT772`](rules/compatibility.md#japiext772) | 'org.springframework.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT773`](rules/compatibility.md#japiext773) | 'org.springframework.dao' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT774`](rules/compatibility.md#japiext774) | 'org.springframework.ejb' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT775`](rules/compatibility.md#japiext775) | 'org.springframework.expression' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT776`](rules/compatibility.md#japiext776) | 'org.springframework.format' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT777`](rules/compatibility.md#japiext777) | 'org.springframework.http' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT778`](rules/compatibility.md#japiext778) | 'org.springframework.instrument' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT779`](rules/compatibility.md#japiext779) | 'org.springframework.jca' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT780`](rules/compatibility.md#japiext780) | 'org.springframework.jdbc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT782`](rules/compatibility.md#japiext782) | 'org.springframework.jmx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT783`](rules/compatibility.md#japiext783) | 'org.springframework.jndi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT784`](rules/compatibility.md#japiext784) | 'org.springframework.lang' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT785`](rules/compatibility.md#japiext785) | 'org.springframework.ldap' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT786`](rules/compatibility.md#japiext786) | 'org.springframework.mail' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT788`](rules/compatibility.md#japiext788) | 'org.springframework.mock' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT789`](rules/compatibility.md#japiext789) | 'org.springframework.objenesis' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT790`](rules/compatibility.md#japiext790) | 'org.springframework.orm' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT792`](rules/compatibility.md#japiext792) | 'org.springframework.remoting' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT793`](rules/compatibility.md#japiext793) | 'org.springframework.scheduling' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT794`](rules/compatibility.md#japiext794) | 'org.springframework.scripting' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT795`](rules/compatibility.md#japiext795) | 'org.springframework.security' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT796`](rules/compatibility.md#japiext796) | 'org.springframework.stereotype' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT797`](rules/compatibility.md#japiext797) | 'org.springframework.test' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT798`](rules/compatibility.md#japiext798) | 'org.springframework.transaction' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT799`](rules/compatibility.md#japiext799) | 'org.springframework.ui' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT800`](rules/compatibility.md#japiext800) | 'org.springframework.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT801`](rules/compatibility.md#japiext801) | 'org.springframework.validation' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT802`](rules/compatibility.md#japiext802) | 'org.springframework.web' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT803`](rules/compatibility.md#japiext803) | 'org.sqlite.core' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT804`](rules/compatibility.md#japiext804) | 'org.sqlite.date' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT805`](rules/compatibility.md#japiext805) | 'org.sqlite.javax' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT806`](rules/compatibility.md#japiext806) | 'org.sqlite.jdbc3' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT807`](rules/compatibility.md#japiext807) | 'org.sqlite.jdbc4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT808`](rules/compatibility.md#japiext808) | 'org.sqlite.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT812`](rules/compatibility.md#japiext812) | 'org.tukaani.xz' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT819`](rules/compatibility.md#japiext819) | 'org.eclipse.cdt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT820`](rules/compatibility.md#japiext820) | 'org.eclipse.jface' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT821`](rules/compatibility.md#japiext821) | 'org.eclipse.swt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT822`](rules/compatibility.md#japiext822) | 'org.eclipse.text' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT823`](rules/compatibility.md#japiext823) | 'com.zaxxer.sparsebits' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT825`](rules/compatibility.md#japiext825) | 'org.abego.treelayout' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT826`](rules/compatibility.md#japiext826) | 'org.antlr.v4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT827`](rules/compatibility.md#japiext827) | 'org.glassfish.json' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT828`](rules/compatibility.md#japiext828) | 'org.stringtemplate.v4' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT830`](rules/compatibility.md#japiext830) | 'org.apache.logging' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT831`](rules/compatibility.md#japiext831) | 'org.aspectj.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT832`](rules/compatibility.md#japiext832) | 'org.aspectj.lang' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT833`](rules/compatibility.md#japiext833) | 'org.h2.mode' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT834`](rules/compatibility.md#japiext834) | 'org.slf4j.event' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT835`](rules/compatibility.md#japiext835) | 'org.apache.felix' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT836`](rules/compatibility.md#japiext836) | 'org.eclipse.equinox' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT837`](rules/compatibility.md#japiext837) | 'org.osgi.dto' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT838`](rules/compatibility.md#japiext838) | 'org.osgi.resource' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT839`](rules/compatibility.md#japiext839) | 'jakarta.xml.ws' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT840`](rules/compatibility.md#japiext840) | 'oracle.core.lmx' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT841`](rules/compatibility.md#japiext841) | 'oracle.core.lvf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT842`](rules/compatibility.md#japiext842) | 'oracle.jdbc.aq' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT843`](rules/compatibility.md#japiext843) | 'oracle.jdbc.babelfish' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT844`](rules/compatibility.md#japiext844) | 'oracle.jdbc.clio' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT845`](rules/compatibility.md#japiext845) | 'oracle.jdbc.connector' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT846`](rules/compatibility.md#japiext846) | 'oracle.jdbc.datasource' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT847`](rules/compatibility.md#japiext847) | 'oracle.jdbc.dcn' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT848`](rules/compatibility.md#japiext848) | 'oracle.jdbc.diagnostics' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT849`](rules/compatibility.md#japiext849) | 'oracle.jdbc.driver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT850`](rules/compatibility.md#japiext850) | 'oracle.jdbc.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT851`](rules/compatibility.md#japiext851) | 'oracle.jdbc.logging' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT852`](rules/compatibility.md#japiext852) | 'oracle.jdbc.oci' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT853`](rules/compatibility.md#japiext853) | 'oracle.jdbc.oracore' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT854`](rules/compatibility.md#japiext854) | 'oracle.jdbc.pool' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT855`](rules/compatibility.md#japiext855) | 'oracle.jdbc.proxy' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT856`](rules/compatibility.md#japiext856) | 'oracle.jdbc.replay' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT857`](rules/compatibility.md#japiext857) | 'oracle.jdbc.spi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT858`](rules/compatibility.md#japiext858) | 'oracle.jdbc.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT859`](rules/compatibility.md#japiext859) | 'oracle.jdbc.xa' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT860`](rules/compatibility.md#japiext860) | 'oracle.jpub.runtime' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT861`](rules/compatibility.md#japiext861) | 'oracle.net.ano' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT862`](rules/compatibility.md#japiext862) | 'oracle.net.aso' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT863`](rules/compatibility.md#japiext863) | 'oracle.net.jdbc' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT864`](rules/compatibility.md#japiext864) | 'oracle.net.jndi' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT865`](rules/compatibility.md#japiext865) | 'oracle.net.mesg' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT866`](rules/compatibility.md#japiext866) | 'oracle.net.ns' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT867`](rules/compatibility.md#japiext867) | 'oracle.net.nt' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT868`](rules/compatibility.md#japiext868) | 'oracle.net.resolver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT869`](rules/compatibility.md#japiext869) | 'oracle.security.o3logon' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT870`](rules/compatibility.md#japiext870) | 'oracle.security.o5logon' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT871`](rules/compatibility.md#japiext871) | 'oracle.sql.converter' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT872`](rules/compatibility.md#japiext872) | 'oracle.sql.json' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT873`](rules/compatibility.md#japiext873) | 'io.prometheus.client' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT874`](rules/compatibility.md#japiext874) | 'com.google.errorprone' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT875`](rules/compatibility.md#japiext875) | 'org.postgresql.jdbcurlresolver' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT876`](rules/compatibility.md#japiext876) | 'org.postgresql.jre7' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT877`](rules/compatibility.md#japiext877) | 'org.postgresql.plugin' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT878`](rules/compatibility.md#japiext878) | 'org.postgresql.replication' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT879`](rules/compatibility.md#japiext879) | 'org.postgresql.shaded' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT880`](rules/compatibility.md#japiext880) | 'org.postgresql.xml' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT881`](rules/compatibility.md#japiext881) | 'io.grpc.internal' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT882`](rules/compatibility.md#japiext882) | 'io.grpc.netty' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT883`](rules/compatibility.md#japiext883) | 'io.grpc.protobuf' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT884`](rules/compatibility.md#japiext884) | 'io.grpc.stub' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |
| [`JAPIEXT885`](rules/compatibility.md#japiext885) | 'io.grpc.util' Java package and subpackages will not be available in MATLAB in a future release. To continue using this package, install its JAR file and add the JAR file to the static path in MATLAB. |

### Suggested Improvements (243 checks)

| Check ID | Message |
| -------- | ------- |
| [`IMPORTDYN`](rules/suggested-improvements.md#importdyn) | Using function syntax to call 'import' is not recommended. With appropriate code changes, use command syntax instead. |
| [`LERR`](rules/suggested-improvements.md#lerr) | LASTERR and LASTERROR are not recommended. Use an identifier on the CATCH block instead. |
| [`EVLC`](rules/suggested-improvements.md#evlc) | Using 'evalc' with two arguments is not recommended. Use try/catch statements instead to make code more clear and efficient. |
| [`RAND`](rules/suggested-improvements.md#rand) | RAND or RANDN with the 'seed', 'state', or 'twister' inputs is not recommended. Use RNG instead. |
| [`HOUGH`](rules/suggested-improvements.md#hough) | HOUGH(BW,'ThetaResolution',VAL) is not recommended. Use HOUGH(BW,'Theta',-90:VAL:(90-VAL)) instead. |
| [`THOUR`](rules/suggested-improvements.md#thour) | 'hour' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| [`TMNTH`](rules/suggested-improvements.md#tmnth) | 'month' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| [`TMNUT`](rules/suggested-improvements.md#tmnut) | 'minute' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| [`TNDAY`](rules/suggested-improvements.md#tnday) | 'day' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| [`TSCND`](rules/suggested-improvements.md#tscnd) | 'second' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| [`TQURT`](rules/suggested-improvements.md#tqurt) | 'quarter' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| [`TYEAR`](rules/suggested-improvements.md#tyear) | 'year' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' as input instead. |
| [`TDTVEC`](rules/suggested-improvements.md#tdtvec) | 'datevec' with serial date number or text inputs is not recommended. With appropriate code changes, use 'datetime' instead. |
| [`TNOW1`](rules/suggested-improvements.md#tnow1) | 'now' is not recommended. With appropriate code changes, use 'datetime(\ |
| [`TNOW2`](rules/suggested-improvements.md#tnow2) | 'datetime(now, 'ConvertFrom', 'datenum')' is not recommended. Use 'datetime(\ |
| [`TTDAY1`](rules/suggested-improvements.md#ttday1) | 'today' is not recommended. With appropriate code changes, use 'datetime(\ |
| [`TTDAY2`](rules/suggested-improvements.md#ttday2) | 'datetime(today, 'ConvertFrom', 'datenum')' is not recommended. Use 'datetime(\ |
| [`MATCH2`](rules/suggested-improvements.md#match2) | STRMATCH is not recommended. Use STRNCMP or VALIDATESTRING instead. |
| [`IMGDT`](rules/suggested-improvements.md#imgdt) | Using 'DataAugmentation' in function 'imageInputLayer' is not recommended. Use function 'augmentedImageDatastore' instead. |
| [`MATCH3`](rules/suggested-improvements.md#match3) | STRMATCH is not recommended. Use STRCMP instead. |
| [`MTFA1`](rules/suggested-improvements.md#mtfa1) | MAKETFORM('AFFINE',A) is not recommended. Use AFFINE2D or AFFINE3D instead. |
| [`MTFA2`](rules/suggested-improvements.md#mtfa2) | MAKETFORM('AFFINE',U,X) is not recommended. Use FITGEOTRANS instead. |
| [`MTFP1`](rules/suggested-improvements.md#mtfp1) | MAKETFORM('PROJECTIVE',A) is not recommended. Use PROJECTIVE2D instead. |
| [`MTFP2`](rules/suggested-improvements.md#mtfp2) | MAKETFORM('PROJECTIVE',U,X) is not recommended. Use FITGEOTRANS instead. |
| [`MTFB`](rules/suggested-improvements.md#mtfb) | MAKETFORM('BOX',...) is not recommended. Use IMREF2D or IMREF3D instead. |
| [`OOPS`](rules/suggested-improvements.md#oops) | Defining a class using 'function' syntax is not recommended. With appropriate code changes, use 'classdef' syntax instead. |
| [`PMTMCONF`](rules/suggested-improvements.md#pmtmconf) | When using PMTM with three output arguments, the 'ConfidenceLevel' input argument is recommended. |
| [`NCHKI`](rules/suggested-improvements.md#nchki) | NARGCHK is not recommended. Use NARGINCHK instead. |
| [`NCHKO`](rules/suggested-improvements.md#nchko) | Using NARGCHK with NARGOUT is not recommended. Use NARGOUTCHK instead. |
| [`NCHKN`](rules/suggested-improvements.md#nchkn) | NARGCHK is not recommended. Use NARGINCHK without ERROR instead. |
| [`NCHKM`](rules/suggested-improvements.md#nchkm) | NARGCHK is not recommended. Use NARGOUTCHK without ERROR instead. |
| [`ISCLSTR`](rules/suggested-improvements.md#isclstr) | To support string in addition to cellstr, include a call to 'isstring'. |
| [`EMTAG`](rules/suggested-improvements.md#emtag) | The compilation directive (or pragma) 'eml' is not recommended. Use 'codegen' instead. |
| [`EMXTR`](rules/suggested-improvements.md#emxtr) | The 'eml' namespace is not recommended. Use 'codegen' instead. |
| [`NVREPLA`](rules/suggested-improvements.md#nvrepla) | 'addParamValue' is not recommended. Use 'addParameter' instead. |
| [`NVREPLM`](rules/suggested-improvements.md#nvreplm) | 'MidPctRef' is not recommended. Use 'MidPercentReferenceLevel' instead. |
| [`NVREPLP`](rules/suggested-improvements.md#nvreplp) | 'PctRefLevels' is not recommended. Use 'PercentReferenceLevels' instead. |
| [`VIDREAD`](rules/suggested-improvements.md#vidread) | 'NumberOfFrames' is not recommended. Use 'NumFrames' instead. |
| [`CRNR`](rules/suggested-improvements.md#crnr) | CORNER is not recommended. Use detectHarrisFeatures or detectMinEigenFeatures in Computer Vision Toolbox instead. |
| [`CRNRM`](rules/suggested-improvements.md#crnrm) | CORNERMETRIC is not recommended. Use detectHarrisFeatures or detectMinEigenFeatures and the cornerPoints class in Computer Vision Toolbox instead. |
| [`MDFLT1`](rules/suggested-improvements.md#mdflt1) | BLKSZ is required for backward compatibility and is ignored. Use [] instead. |
| [`INTRPP`](rules/suggested-improvements.md#intrpp) | 'pp' is not recommended. Use the griddedInterpolant class instead. |
| [`DISPLAYPROG`](rules/suggested-improvements.md#displayprog) | Programmatic use of DISPLAY is not recommended. Use DISP or FPRINTF instead. |
| [`HGSTGT`](rules/suggested-improvements.md#hgstgt) | hgsetget is not recommended. Use matlab.mixin.SetGet or matlab.mixin.SetGetExactNames instead. |
| [`LEGACYMD`](rules/suggested-improvements.md#legacymd) | Setting LegacyMode to true is not recommended. Set LegacyMode to false instead. |
| [`LEGACYTRD`](rules/suggested-improvements.md#legacytrd) | 'DetectorMethod' is applicable only when LegacyMode is set to true. However, setting LegacyMode to true is not recommended. |
| [`LEGACYTRL`](rules/suggested-improvements.md#legacytrl) | 'LoopMethod' is applicable only when LegacyMode is set to true. However, setting LegacyMode to true is not recommended. |
| [`LEGACYTRU`](rules/suggested-improvements.md#legacytru) | 'UpdatePeriod' is applicable only when LegacyMode is set to true. However, setting LegacyMode to true is not recommended. |
| [`LEGACYTRS`](rules/suggested-improvements.md#legacytrs) | 'StepSize' is applicable only when LegacyMode is set to true. However, setting LegacyMode to true is not recommended. |
| [`LEGACYTRG`](rules/suggested-improvements.md#legacytrg) | 'GainOutputPort' is applicable only when LegacyMode is set to true. However, setting LegacyMode to true is not recommended. |
| [`EZPLT`](rules/suggested-improvements.md#ezplt) | EZPLOT is not recommended. Use FPLOT or FIMPLICIT instead. |
| [`EZGRPH3`](rules/suggested-improvements.md#ezgrph3) | EZGRAPH3 is not recommended. Use FCONTOUR, FMESH, FPLOT, FPLOT3 or FSURF instead. |
| [`EZCNTRF`](rules/suggested-improvements.md#ezcntrf) | EZCONTOURF is not recommended. Use FCONTOUR instead, and set the 'Fill' value to 'on'. |
| [`EZMSHC`](rules/suggested-improvements.md#ezmshc) | EZMESHC is not recommended. Use FMESH instead, and set the 'ShowContours' value to 'on'. |
| [`EZSRFC`](rules/suggested-improvements.md#ezsrfc) | EZSURFC is not recommended. Use FSURF instead, and set the 'ShowContours' value to 'on'. |
| [`FISADR`](rules/suggested-improvements.md#fisadr) | 'addrule' is not recommended. Use 'addRule' instead. |
| [`STRQUOT`](rules/suggested-improvements.md#strquot) | string('...') is not recommended. Use \ |
| [`STRCLQT`](rules/suggested-improvements.md#strclqt) | 'string({''str1'', ''str2''})' is not recommended. Use '[\ |
| [`SIM`](rules/suggested-improvements.md#sim) | 'sim' in parfor loop is not recommended. Replace the parfor loop with 'parsim'. |
| [`NUMCH`](rules/suggested-improvements.md#numch) | 'NumberOfChannels' is not recommended. Use 'NumChannels' instead. |
| [`GTRED`](rules/suggested-improvements.md#gtred) | 'geotiffread' is not recommended, except when reading a GeoTIFF file from a URL. With appropriate code changes, use 'readgeoraster' instead. |
| [`GETFSP`](rules/suggested-improvements.md#getfsp) | Using get for retrieving values of line spacing is not recommended. With appropriate code changes, use 'settings' object instead. |
| [`SETFMT`](rules/suggested-improvements.md#setfmt) | Using set for assigning values of numeric display format is not recommended. With appropriate code changes, use 'settings' object instead. |
| [`SETFSP`](rules/suggested-improvements.md#setfsp) | Using set for assigning values of line spacing is not recommended. With appropriate code changes, use 'settings' object instead. |
| [`GETFMT`](rules/suggested-improvements.md#getfmt) | Using get for retrieving values of numeric display format is not recommended. With appropriate code changes, use 'settings' object instead. |
| [`EV2IN`](rules/suggested-improvements.md#ev2in) | Using 'eval' with two arguments is not recommended. Use try/catch statements instead to make code more clear and efficient. |
| [`EV3IN`](rules/suggested-improvements.md#ev3in) | Using 'evalin' with three arguments is not recommended. Use try/catch statements instead to make code more clear and efficient. |
| [`PRTOG`](rules/suggested-improvements.md#prtog) | '-opengl' is not recommended. Use '-image' instead, which is a direct replacement. |
| [`PRTPT`](rules/suggested-improvements.md#prtpt) | '-painters' is not recommended. Use '-vector' instead, which is a direct replacement. |
| [`FORMATNOI`](rules/suggested-improvements.md#formatnoi) | 'format' with no input or output arguments is not recommended. Use 'format(\ |
| [`XFRWSB`](rules/suggested-improvements.md#xfrwsb) | The 'TransferBaseWorkspaceVariables' option is not recommended for 'batchsim'. With appropriate code changes, consider using project startup scripts or the 'SetupFcn' option instead. |
| [`XFRWSP`](rules/suggested-improvements.md#xfrwsp) | The 'TransferBaseWorkspaceVariables' option is not recommended for 'parsim'. With appropriate code changes, consider using project startup scripts or the 'SetupFcn' option instead. |
| [`OLDSIM`](rules/suggested-improvements.md#oldsim) | This syntax of the 'sim' command which returns multiple arguments is not recommended. With appropriate code changes, turn on 'ReturnWorkspaceOutputs' and return simulation results using the single-output format instead. |
| [`INSTHWI`](rules/suggested-improvements.md#insthwi) | 'instrhwinfo('ivi')' is not recommended. With appropriate code changes, use 'ividriverlist' or 'ividevlist' instead. |
| [`INWVX`](rules/suggested-improvements.md#inwvx) | 'instrhwinfo('vxipnp')' is not recommended. With appropriate code changes, use 'ividriverlist' or 'ividevlist' instead. |
| [`VERMATLAB`](rules/suggested-improvements.md#vermatlab) | ver('matlab') is not recommended. With appropriate code changes, use 'matlabRelease' instead. |
| [`VERLESSMATLAB`](rules/suggested-improvements.md#verlessmatlab) | verLessThan('matlab', ...) is not recommended. With appropriate code changes, use 'isMATLABReleaseOlderThan' instead. |
| [`RISKLMM`](rules/suggested-improvements.md#risklmm) | 'Model' property of 'risk.credit.pd.LifetimePDModel' class is not recommended. Use 'UnderlyingModel' property of 'risk.credit.pd.LifetimePDModel' class instead. |
| [`RISKLMA`](rules/suggested-improvements.md#risklma) | 'modelAccuracy' method of 'risk.credit.pd.LifetimePDModel' class is not recommended. Use 'modelCalibration' method of 'risk.credit.pd.LifetimePDModel' class instead. |
| [`RISKLMAP`](rules/suggested-improvements.md#risklmap) | 'modelAccuracyPlot' method of 'risk.credit.pd.LifetimePDModel' class is not recommended. Use 'modelCalibrationPlot' method of 'risk.credit.pd.LifetimePDModel' class instead. |
| [`RISKLGDMA`](rules/suggested-improvements.md#risklgdma) | 'modelAccuracy' method of 'risk.credit.lgd.LGDModel' class is not recommended. Use 'modelCalibration' method of 'risk.credit.lgd.LGDModel' class instead. |
| [`RISKLGDMAP`](rules/suggested-improvements.md#risklgdmap) | 'modelAccuracyPlot' method of 'risk.credit.lgd.LGDModel' class is not recommended. Use 'modelCalibrationPlot' method of 'risk.credit.lgd.LGDModel' class instead. |
| [`RISKEADMA`](rules/suggested-improvements.md#riskeadma) | 'modelAccuracy' method of 'risk.credit.ead.EADModel' class is not recommended. Use 'modelCalibration' method of 'risk.credit.ead.EADModel' class instead. |
| [`RISKEADMAP`](rules/suggested-improvements.md#riskeadmap) | 'modelAccuracyPlot' method of 'risk.credit.ead.EADModel' class is not recommended. Use 'modelCalibrationPlot' method of 'risk.credit.ead.EADModel' class instead. |
| [`HOLDALL`](rules/suggested-improvements.md#holdall) | 'hold('all')' is not recommended. Use 'hold('on')' instead, which is a direct replacement. |
| [`MASSO`](rules/suggested-improvements.md#masso) | 'MasterSolverOptions' is not recommended. Use 'MainSolverOptions' instead, which is a direct replacement. |
| [`IMSSO`](rules/suggested-improvements.md#imsso) | 'IntMasterSolverOptions' is not recommended. Use 'IntMainSolverOptions' instead, which is a direct replacement. |
| [`CDFEPOCH2DATE`](rules/suggested-improvements.md#cdfepoch2date) | 'ConvertEpochToDatenum' is not recommended. With appropriate code changes, use the 'DatetimeType' parameter of 'cdfread' instead. |
| [`FEATGPID`](rules/suggested-improvements.md#featgpid) | 'feature('getpid')' is unsupported and not recommended. With appropriate code changes, use the function 'matlabProcessID' instead. |
| [`DTRIREP`](rules/suggested-improvements.md#dtrirep) | 'TriRep' is not recommended. With appropriate code changes, use 'triangulation' instead. |
| [`DDELTRI`](rules/suggested-improvements.md#ddeltri) | 'DelaunayTri' is not recommended. With appropriate code changes, use 'delaunayTriangulation' instead. |
| [`DTRIINT`](rules/suggested-improvements.md#dtriint) | 'TriScatteredInterp' is not recommended. With appropriate code changes, use 'scatteredInterpolant' instead. |
| [`DAPPLUT`](rules/suggested-improvements.md#dapplut) | 'applylut' is not recommended. With appropriate code changes, use 'bwlookup' instead. |
| [`DBLKPRC`](rules/suggested-improvements.md#dblkprc) | 'blkproc' is not recommended. With appropriate code changes, use 'blockproc' instead. |
| [`CAXIS`](rules/suggested-improvements.md#caxis) | 'caxis' is not recommended. Use 'clim' instead, which is a direct replacement. |
| [`CDFEPOCH`](rules/suggested-improvements.md#cdfepoch) | 'cdfepoch' is not recommended. With appropriate code changes, use 'cdflib' low-level functions instead. |
| [`TODATENUM`](rules/suggested-improvements.md#todatenum) | 'todatenum' is not recommended. With appropriate code changes, use the 'DatetimeType' parameter of 'cdfread' instead. |
| [`COMMPAMM`](rules/suggested-improvements.md#commpamm) | 'comm.PAMModulator' is not recommended. With appropriate code changes, use 'pammod' instead. |
| [`COMMPAMD`](rules/suggested-improvements.md#commpamd) | 'comm.PAMDemodulator' is not recommended. With appropriate code changes, use 'pamdemod' instead. |
| [`COMMSRC`](rules/suggested-improvements.md#commsrc) | 'commsrc.pn' is not recommended. With appropriate code changes, use 'comm.PNSequence' instead. |
| [`DCPTF`](rules/suggested-improvements.md#dcptf) | 'cp2tform' is not recommended. With appropriate code changes, use 'fitgeotrans' instead. |
| [`DATNM`](rules/suggested-improvements.md#datnm) | 'datenum' is not recommended. With appropriate code changes, use 'datetime' instead. |
| [`DATST`](rules/suggested-improvements.md#datst) | 'datestr' is not recommended. With appropriate code changes, use 'datetime' instead. |
| [`DETIM`](rules/suggested-improvements.md#detim) | 'etime' is not recommended. With appropriate code changes, use 'datetime' and the minus operator instead. |
| [`DATOD`](rules/suggested-improvements.md#datod) | 'addtodate' is not recommended. With appropriate code changes, use 'datetime', 'duration', and the plus operator instead. |
| [`CLOCK`](rules/suggested-improvements.md#clock) | 'clock' is not recommended. With appropriate code changes, use 'datetime(\ |
| [`DATE`](rules/suggested-improvements.md#date) | 'date' is not recommended. With appropriate code changes, use 'datetime(\ |
| [`DATIC`](rules/suggested-improvements.md#datic) | 'datetick' is not recommended. With appropriate code changes, use datetime and duration arrays directly in charts. Modify display using 'xtickformat', 'ytickformat', or 'ztickformat'. |
| [`WKNUM`](rules/suggested-improvements.md#wknum) | 'weeknum' is not recommended. With appropriate code changes, use 'week' with a 'datetime' input instead. |
| [`EMDATE`](rules/suggested-improvements.md#emdate) | 'eomdate' is not recommended. With appropriate code changes, use 'dateshift' with a 'datetime' input instead. |
| [`XMDATE`](rules/suggested-improvements.md#xmdate) | 'x2mdate' is not recommended. With appropriate code changes, use 'datetime(..., \ |
| [`MXDATE`](rules/suggested-improvements.md#mxdate) | 'm2xdate' is not recommended. With appropriate code changes, use 'exceltime' with a 'datetime' input instead. |
| [`MNTHS`](rules/suggested-improvements.md#mnths) | 'months' is not recommended. With appropriate code changes, use 'between' with a 'datetime' input instead. |
| [`DGTORD`](rules/suggested-improvements.md#dgtord) | 'degtorad' is not recommended. Use 'deg2rad' instead, which is a direct replacement. |
| [`RDTODG`](rules/suggested-improvements.md#rdtodg) | 'radtodeg' is not recommended. Use 'rad2deg' instead, which is a direct replacement. |
| [`EZCONTR`](rules/suggested-improvements.md#ezcontr) | 'ezcontour' is not recommended. With appropriate code changes, use 'fcontour' instead. |
| [`EZMESH`](rules/suggested-improvements.md#ezmesh) | 'ezmesh' is not recommended. With appropriate code changes, use 'fmesh' instead. |
| [`EZPLT3`](rules/suggested-improvements.md#ezplt3) | 'ezplot3' is not recommended. With appropriate code changes, use 'fplot3' instead. |
| [`EZSURF`](rules/suggested-improvements.md#ezsurf) | 'ezsurf' is not recommended. With appropriate code changes, use 'fsurf' instead. |
| [`EZPOLAR`](rules/suggested-improvements.md#ezpolar) | 'ezpolar' is not recommended. With appropriate code changes, use 'fpolarplot' instead. |
| [`COMPASS`](rules/suggested-improvements.md#compass) | 'compass' is not recommended. With appropriate code changes, use 'compassplot' instead. |
| [`DFLIPDIM`](rules/suggested-improvements.md#dflipdim) | 'flipdim' is not recommended. With appropriate code changes, use 'flip' instead. |
| [`DSTRMT`](rules/suggested-improvements.md#dstrmt) | 'str2mat' is not recommended. With appropriate code changes, use 'char' instead. |
| [`DSTSTR`](rules/suggested-improvements.md#dststr) | 'setstr' is not recommended. With appropriate code changes, use 'char' instead. |
| [`DSTRVCT`](rules/suggested-improvements.md#dstrvct) | 'strvcat' is not recommended. With appropriate code changes, use 'char' instead. |
| [`DISSTR`](rules/suggested-improvements.md#disstr) | 'isstr' is not recommended. With appropriate code changes, use 'ischar' instead. |
| [`DFTSMTX`](rules/suggested-improvements.md#dftsmtx) | 'fts2mtx' is not recommended. With appropriate code changes, use 'fts2mat' instead. |
| [`FISWRT`](rules/suggested-improvements.md#fiswrt) | 'writefis' is not recommended. Use 'writeFIS' instead, which is a direct replacement. |
| [`FISADM`](rules/suggested-improvements.md#fisadm) | 'addmf' is not recommended. With appropriate code changes, use 'addMF' instead. |
| [`HIST`](rules/suggested-improvements.md#hist) | 'hist' is not recommended. With appropriate code changes, use 'histogram' instead. |
| [`HISTC`](rules/suggested-improvements.md#histc) | 'histc' is not recommended. With appropriate code changes, use 'histcounts' instead. |
| [`ROSE`](rules/suggested-improvements.md#rose) | 'rose' is not recommended. With appropriate code changes, use 'polarhistogram' instead. |
| [`H5CLS`](rules/suggested-improvements.md#h5cls) | 'H5.close' is not recommended and no longer has any effect. There is no simple replacement for this. |
| [`H5OPN`](rules/suggested-improvements.md#h5opn) | 'H5.open' is not recommended and no longer has any effect. There is no simple replacement for this. |
| [`HDFI`](rules/suggested-improvements.md#hdfi) | 'hdf5info' is not recommended. With appropriate code changes, use 'h5info' instead. |
| [`HDFW`](rules/suggested-improvements.md#hdfw) | 'hdf5write' is not recommended. With appropriate code changes, use 'h5write' instead. |
| [`HDFR`](rules/suggested-improvements.md#hdfr) | 'hdf5read' is not recommended. With appropriate code changes, use 'h5read' instead. |
| [`IM2BW`](rules/suggested-improvements.md#im2bw) | 'im2bw' is not recommended. With appropriate code changes, use 'imbinarize' instead. |
| [`ISPIX`](rules/suggested-improvements.md#ispix) | 'pixelLabelImageSource' is not recommended. Use 'pixelLabelImageDatastore' instead, which is a direct replacement. |
| [`ISAUG`](rules/suggested-improvements.md#isaug) | 'augmentedImageSource' is not recommended. Use 'augmentedImageDatastore' instead, which is a direct replacement. |
| [`ISDNS`](rules/suggested-improvements.md#isdns) | 'denoisingImageSource' is not recommended. Use 'denoisingImageDatastore' instead, which is a direct replacement. |
| [`IMFREEH`](rules/suggested-improvements.md#imfreeh) | 'imfreehand' is not recommended. With appropriate code changes, use 'drawfreehand' instead. |
| [`IMRECT`](rules/suggested-improvements.md#imrect) | 'imrect' is not recommended. With appropriate code changes, use 'drawrectangle' instead. |
| [`IMLINE`](rules/suggested-improvements.md#imline) | 'imline' is not recommended. With appropriate code changes, use 'drawline' instead. |
| [`IMPNT`](rules/suggested-improvements.md#impnt) | 'impoint' is not recommended. With appropriate code changes, use 'drawpoint' instead. |
| [`IMPOLY`](rules/suggested-improvements.md#impoly) | 'impoly' is not recommended. With appropriate code changes, use 'drawpolygon' or 'drawpolyline' instead. |
| [`IMELLPS`](rules/suggested-improvements.md#imellps) | 'imellipse' is not recommended. With appropriate code changes, use 'drawellipse' or 'drawcircle' instead. |
| [`DIMTRNS`](rules/suggested-improvements.md#dimtrns) | 'imtransform' is not recommended. With appropriate code changes, use 'imwarp' instead. |
| [`FILEATTRIB`](rules/suggested-improvements.md#fileattrib) | 'fileattrib' is not recommended. With appropriate code changes, use 'filePermissions' instead. |
| [`CSVRD`](rules/suggested-improvements.md#csvrd) | 'csvread' is not recommended. With appropriate code changes, use 'readtable' or 'readmatrix' instead. |
| [`DLMRD`](rules/suggested-improvements.md#dlmrd) | 'dlmread' is not recommended. With appropriate code changes, use 'readtable' or 'readmatrix' instead. |
| [`CSVWT`](rules/suggested-improvements.md#csvwt) | 'csvwrite' is not recommended. With appropriate code changes, use 'writematrix' instead. |
| [`DLMWT`](rules/suggested-improvements.md#dlmwt) | 'dlmwrite' is not recommended. With appropriate code changes, use 'writematrix' instead. |
| [`XLSRD`](rules/suggested-improvements.md#xlsrd) | 'xlsread' is not recommended. With appropriate code changes, use 'readtable', 'readmatrix' or 'readcell' instead. |
| [`XLSWT`](rules/suggested-improvements.md#xlswt) | 'xlswrite' is not recommended. With appropriate code changes, use 'writematrix' or 'writecell' instead. |
| [`ISDIR`](rules/suggested-improvements.md#isdir) | 'isdir' is not recommended. Use 'isfolder' instead, which is a direct replacement. |
| [`DISEQN`](rules/suggested-improvements.md#diseqn) | 'isequalwithequalnans' is not recommended. With appropriate code changes, use 'isequaln' instead. |
| [`DGCAT`](rules/suggested-improvements.md#dgcat) | 'gcat' is not recommended. Use 'spmdCat' instead, which is a direct replacement. |
| [`DGOP`](rules/suggested-improvements.md#dgop) | 'gop' is not recommended. Use 'spmdReduce' instead, which is a direct replacement. |
| [`DGPLUS`](rules/suggested-improvements.md#dgplus) | 'gplus' is not recommended. Use 'spmdPlus' instead, which is a direct replacement. |
| [`DLABBARRIER`](rules/suggested-improvements.md#dlabbarrier) | 'labBarrier' is not recommended. Use 'spmdBarrier' instead, which is a direct replacement. |
| [`DLABBROADCAST`](rules/suggested-improvements.md#dlabbroadcast) | 'labBroadcast' is not recommended. Use 'spmdBroadcast' instead, which is a direct replacement. |
| [`DLABINDEX`](rules/suggested-improvements.md#dlabindex) | 'labindex' is not recommended. Use 'spmdIndex' instead, which is a direct replacement. |
| [`DLABPROBE`](rules/suggested-improvements.md#dlabprobe) | 'labProbe' is not recommended. Use 'spmdProbe' instead, which is a direct replacement. |
| [`DLABRECEIVE`](rules/suggested-improvements.md#dlabreceive) | 'labReceive' is not recommended. Use 'spmdReceive' instead, which is a direct replacement. |
| [`DLABSEND`](rules/suggested-improvements.md#dlabsend) | 'labSend' is not recommended. Use 'spmdSend' instead, which is a direct replacement. |
| [`DLABSENDRECEIVE`](rules/suggested-improvements.md#dlabsendreceive) | 'labSendReceive' is not recommended. Use 'spmdSendReceive' instead, which is a direct replacement. |
| [`DNUMLABS`](rules/suggested-improvements.md#dnumlabs) | 'numlabs' is not recommended. Use 'spmdSize' instead, which is a direct replacement. |
| [`AGRED`](rules/suggested-improvements.md#agred) | 'arcgridread' is not recommended. With appropriate code changes, use 'readgeoraster' instead. |
| [`MDFOBJ`](rules/suggested-improvements.md#mdfobj) | 'mdf' is not recommended. With appropriate code changes, use 'mdfInfo', 'mdfChannelGroupInfo' or 'mdfChannelInfo' instead. |
| [`DDBMEX`](rules/suggested-improvements.md#ddbmex) | 'mexdebug' is not recommended. With appropriate code changes, use 'dbmex' instead. |
| [`MLNT`](rules/suggested-improvements.md#mlnt) | 'mlint' is not recommended. Use 'checkcode' instead, which is a direct replacement. |
| [`MNRFIT`](rules/suggested-improvements.md#mnrfit) | 'mnrfit' is not recommended. With appropriate code changes, use 'fitmnr' instead. |
| [`MNRVAL`](rules/suggested-improvements.md#mnrval) | 'mnrval' is not recommended. With appropriate code changes, use 'MultinomialRegression.predict' instead. |
| [`MUSTINRANGE`](rules/suggested-improvements.md#mustinrange) | 'mustBeInRange' is not recommended. With appropriate code changes, use 'mustBeBetween' instead. |
| [`DEPNOE`](rules/suggested-improvements.md#depnoe) | 'numberofelements' is not recommended. With appropriate code changes, use 'numel' instead. |
| [`GAOPT`](rules/suggested-improvements.md#gaopt) | 'gaoptimset' is not recommended. With appropriate code changes, use 'optimoptions' instead. |
| [`PSOPT`](rules/suggested-improvements.md#psopt) | 'psoptimset' is not recommended. With appropriate code changes, use 'optimoptions' instead. |
| [`SAOPT`](rules/suggested-improvements.md#saopt) | 'saoptimset' is not recommended. With appropriate code changes, use 'optimoptions' instead. |
| [`PLOTYY`](rules/suggested-improvements.md#plotyy) | 'plotyy' is not recommended. With appropriate code changes, use 'yyaxis' instead. |
| [`POLAR`](rules/suggested-improvements.md#polar) | 'polar' (MATLAB) is not recommended. Use 'polarplot' instead. |
| [`PLBL`](rules/suggested-improvements.md#plbl) | 'polybool' is not recommended. With appropriate code changes, use 'polyshape' instead. |
| [`PYVER`](rules/suggested-improvements.md#pyver) | 'pyversion' is not recommended. With appropriate code changes, use 'pyenv' instead. |
| [`DQUAD`](rules/suggested-improvements.md#dquad) | 'quad' is not recommended. With appropriate code changes, use 'integral' instead. |
| [`DQUADL`](rules/suggested-improvements.md#dquadl) | 'quadl' is not recommended. With appropriate code changes, use 'integral' instead. |
| [`DQUADV`](rules/suggested-improvements.md#dquadv) | 'quadv' is not recommended. With appropriate code changes, use 'integral' instead. |
| [`DDBLQD`](rules/suggested-improvements.md#ddblqd) | 'dblquad' is not recommended. With appropriate code changes, use 'integral2' instead. |
| [`DTRIQD`](rules/suggested-improvements.md#dtriqd) | 'triplequad' is not recommended. With appropriate code changes, use 'integral3' instead. |
| [`DFIRRCOS`](rules/suggested-improvements.md#dfirrcos) | 'firrcos' is not recommended. With appropriate code changes, use 'rcosdesign' instead. |
| [`DFIRGAUSS`](rules/suggested-improvements.md#dfirgauss) | 'firgauss' is not recommended. With appropriate code changes, use 'gaussdesign' instead. |
| [`DGAUSSFIR`](rules/suggested-improvements.md#dgaussfir) | 'gaussfir' is not recommended. With appropriate code changes, use 'gaussdesign' instead. |
| [`ROIFILL`](rules/suggested-improvements.md#roifill) | 'roifill' is not recommended. With appropriate code changes, use 'regionfill' instead. |
| [`DEPBART`](rules/suggested-improvements.md#depbart) | 'sigwin.barthannwin' is not recommended. With appropriate code changes, use 'barthannwin' instead. |
| [`DEPLETT`](rules/suggested-improvements.md#deplett) | 'sigwin.bartlett' is not recommended. With appropriate code changes, use 'bartlett' instead. |
| [`DBLKMN`](rules/suggested-improvements.md#dblkmn) | 'sigwin.blackman' is not recommended. With appropriate code changes, use 'blackman' instead. |
| [`DBHRRS`](rules/suggested-improvements.md#dbhrrs) | 'sigwin.blackmanharris' is not recommended. With appropriate code changes, use 'blackmanharris' instead. |
| [`DBHMNWN`](rules/suggested-improvements.md#dbhmnwn) | 'sigwin.bohmanwin' is not recommended. With appropriate code changes, use 'bohmanwin' instead. |
| [`DCHBWN`](rules/suggested-improvements.md#dchbwn) | 'sigwin.chebwin' is not recommended. With appropriate code changes, use 'chebwin' instead. |
| [`DFLTTPWN`](rules/suggested-improvements.md#dflttpwn) | 'sigwin.flattopwin' is not recommended. With appropriate code changes, use 'flattopwin' instead. |
| [`DGSWIN`](rules/suggested-improvements.md#dgswin) | 'sigwin.gausswin' is not recommended. With appropriate code changes, use 'gausswin' instead. |
| [`DHMMNG`](rules/suggested-improvements.md#dhmmng) | 'sigwin.hamming' is not recommended. With appropriate code changes, use 'hamming' instead. |
| [`DHANN`](rules/suggested-improvements.md#dhann) | 'sigwin.hann' is not recommended. With appropriate code changes, use 'hann' instead. |
| [`DKSER`](rules/suggested-improvements.md#dkser) | 'sigwin.kaiser' is not recommended. With appropriate code changes, use 'kaiser' instead. |
| [`DNLWN`](rules/suggested-improvements.md#dnlwn) | 'sigwin.nuttallwin' is not recommended. With appropriate code changes, use 'nuttallwin' instead. |
| [`DPNWN`](rules/suggested-improvements.md#dpnwn) | 'sigwin.parzenwin' is not recommended. With appropriate code changes, use 'parzenwin' instead. |
| [`DRCTWN`](rules/suggested-improvements.md#drctwn) | 'sigwin.rectwin' is not recommended. With appropriate code changes, use 'rectwin' instead. |
| [`DTYLRWN`](rules/suggested-improvements.md#dtylrwn) | 'sigwin.taylorwin' is not recommended. With appropriate code changes, use 'taylorwin' instead. |
| [`DTRNG`](rules/suggested-improvements.md#dtrng) | 'sigwin.triang' is not recommended. With appropriate code changes, use 'triang' instead. |
| [`DTKYWN`](rules/suggested-improvements.md#dtkywn) | 'sigwin.tukeywin' is not recommended. With appropriate code changes, use 'tukeywin' instead. |
| [`DBURG`](rules/suggested-improvements.md#dburg) | 'spectrum.burg' is not recommended. With appropriate code changes, use 'pburg' instead. |
| [`DCOV`](rules/suggested-improvements.md#dcov) | 'spectrum.cov' is not recommended. With appropriate code changes, use 'pcov' instead. |
| [`DEVCTR`](rules/suggested-improvements.md#devctr) | 'spectrum.eigenvector' is not recommended. With appropriate code changes, use 'peig' instead. |
| [`DMCOV`](rules/suggested-improvements.md#dmcov) | 'spectrum.mcov' is not recommended. With appropriate code changes, use 'pmcov' instead. |
| [`DMTM`](rules/suggested-improvements.md#dmtm) | 'spectrum.mtm' is not recommended. With appropriate code changes, use 'pmtm' instead. |
| [`DMUSIC`](rules/suggested-improvements.md#dmusic) | 'spectrum.music' is not recommended. With appropriate code changes, use 'pmusic' instead. |
| [`DPRDGRM`](rules/suggested-improvements.md#dprdgrm) | 'spectrum.periodogram' is not recommended. With appropriate code changes, use 'periodogram' instead. |
| [`DWELCH`](rules/suggested-improvements.md#dwelch) | 'spectrum.welch' is not recommended. With appropriate code changes, use 'pwelch' instead. |
| [`DYULEAR`](rules/suggested-improvements.md#dyulear) | 'spectrum.yulear' is not recommended. With appropriate code changes, use 'pyulear' instead. |
| [`SIMVARIANT`](rules/suggested-improvements.md#simvariant) | 'Simulink.Variant' is not recommended. Use 'Simulink.VariantExpression' instead, which is a direct replacement. |
| [`COMBNK`](rules/suggested-improvements.md#combnk) | 'combnk' is not recommended. With appropriate code changes, use 'nchoosek' instead. |
| [`NANMEAN`](rules/suggested-improvements.md#nanmean) | 'nanmean' is not recommended. With appropriate code changes, use 'mean' instead. |
| [`NANMEDIAN`](rules/suggested-improvements.md#nanmedian) | 'nanmedian' is not recommended. With appropriate code changes, use 'median' instead. |
| [`NANMAX`](rules/suggested-improvements.md#nanmax) | 'nanmax' is not recommended. With appropriate code changes, use 'max' instead. |
| [`NANMIN`](rules/suggested-improvements.md#nanmin) | 'nanmin' is not recommended. With appropriate code changes, use 'min' instead. |
| [`NANSTD`](rules/suggested-improvements.md#nanstd) | 'nanstd' is not recommended. With appropriate code changes, use 'std' instead. |
| [`NANVAR`](rules/suggested-improvements.md#nanvar) | 'nanvar' is not recommended. With appropriate code changes, use 'var' instead. |
| [`NANCOV`](rules/suggested-improvements.md#nancov) | 'nancov' is not recommended. With appropriate code changes, use 'cov' instead. |
| [`NANSUM`](rules/suggested-improvements.md#nansum) | 'nansum' is not recommended. With appropriate code changes, use 'sum' instead. |
| [`CELLDTSET`](rules/suggested-improvements.md#celldtset) | 'cell2dataset' is not recommended. With appropriate code changes, use 'cell2table' instead. |
| [`DTSET`](rules/suggested-improvements.md#dtset) | 'dataset' is not recommended. With appropriate code changes, use 'table' instead. |
| [`MATDTSET`](rules/suggested-improvements.md#matdtset) | 'mat2dataset' is not recommended. With appropriate code changes, use 'array2table' instead. |
| [`STRUCTDTSET`](rules/suggested-improvements.md#structdtset) | 'struct2dataset' is not recommended. With appropriate code changes, use 'struct2table' instead. |
| [`FSTR`](rules/suggested-improvements.md#fstr) | 'findstr' is not recommended. With appropriate code changes, use 'strfind' instead. |
| [`DSTRRD`](rules/suggested-improvements.md#dstrrd) | 'strread' is not recommended. With appropriate code changes, use 'textscan' instead. |
| [`DTXTRD`](rules/suggested-improvements.md#dtxtrd) | 'textread' is not recommended. With appropriate code changes, use 'textscan' instead. |
| [`SUBIMGNR`](rules/suggested-improvements.md#subimgnr) | 'subimage' is not recommended. With appropriate code changes, use 'imshow' instead. |
| [`URLWR`](rules/suggested-improvements.md#urlwr) | 'urlwrite' is not recommended. With appropriate code changes, use 'websave' instead. |
| [`URLRD`](rules/suggested-improvements.md#urlrd) | 'urlread' is not recommended. With appropriate code changes, use 'webread' or 'webwrite' instead. |
| [`VEMAT`](rules/suggested-improvements.md#vemat) | 'vec2mat' is not recommended. With appropriate code changes, use 'reshape' instead. |
| [`MDFRD`](rules/suggested-improvements.md#mdfrd) | 'read' method of 'mdf' class is not recommended. With appropriate code changes, use 'mdfRead' function instead. |
| [`MDFCHL`](rules/suggested-improvements.md#mdfchl) | 'channelList' method of 'mdf' class is not recommended. With appropriate code changes, use 'mdfChannelInfo' function instead. |
| [`MDFSVA`](rules/suggested-improvements.md#mdfsva) | 'saveAttachment' method of 'mdf' class is not recommended. With appropriate code changes, use 'mdfSaveAttachment' function instead. |
| [`MINELV`](rules/suggested-improvements.md#minelv) | 'MinElevationAngle' is not recommended. Use 'MaskElevationAngle' instead, which is a direct replacement when specified as scalar values. Row vector values must be transposed. |

## Rule Categories

mlt organizes rules into the same categories as MATLAB's Code Analyzer:

| Category | Config Key | Description |
| -------- | ---------- | ----------- |
| Incomplete Analysis | `incomplete-analysis` | Internal linter limits and analysis failures |
| Syntax Errors | `syntax-errors` | Parser-level syntax validation |
| Language Specification | `language-specification` | Language constraint violations |
| Bugs | `bugs` | Likely bugs and logic errors |
| Custom Checks | `custom-checks` | Configurable code complexity/style metrics |
| Naming | `naming` | Naming convention enforcement |
| Compatibility | `compatibility` | Deprecated/removed functions and APIs |
| Forward Compatibility | `forward-compatibility` | Forward compatibility issues |
| Good Practices | `good-practices` | Common best practices |
| Unset Variables | `unset-variables` | Variables that may not be defined before use |
| Unused Constructions | `unused-constructions` | Dead code and unused constructions |
| Suggested Improvements | `suggested-improvements` | Suggestions for improved code patterns |
| Readability | `readability` | Readability improvements |
| Formatting | `formatting` | Code formatting suggestions |
| Performance | `performance` | Performance improvement suggestions |
| Code Generation | `code-generation` | MATLAB Coder constraints |
| Fixed-Point | `fixed-point` | Fixed-point toolbox specific |
| Deployment | `deployment` | MATLAB Compiler deployment constraints |
| System Objects | `system-objects` | System object validation |
| Unsupported | `unsupported` | Unsupported features |
| Behavior Changes | `behavior-changes` | Behavior changes between MATLAB versions |
| Configuration Issues | `configuration-issues` | Configuration file validation |

## Severity Levels

Rules are categorized into three severity levels:

### Error

Critical issues that likely indicate bugs or broken code:

- Code that will fail at runtime
- Language specification violations
- Syntax errors

### Warning

Issues that affect code quality:

- Common bug patterns
- Good practice violations
- Deprecated function usage

### Info

Low-priority suggestions:

- Formatting preferences
- Performance improvement hints
- Readability suggestions

### Configuring Severity

Override default severities in `.mlt.toml`:

```toml
[lint.rules]
NOSEMI = "error"    # Upgrade from info to error
```

Or in a full table:

```toml
[lint.rules.NOSEMI]
severity = "error"
ignore_functions = ["disp", "fprintf"]
```

### Category-Level Configuration

Enable, disable, or override severity for entire categories:

```toml
[lint.categories]
performance = "off"           # Disable all performance rules
compatibility = "warn"        # Override all compatibility rules to warning
formatting = "info"           # Set all formatting rules to info
```

Per-rule configuration always takes precedence over category-level settings.

See [Configuration](configuration.md) for full details.

## Enabling and Disabling Rules

All rules are enabled by default. Disable individual rules:

```toml
[lint.rules]
NOSEMI = "off"
```

Or disable entire categories:

```toml
[lint.categories]
behavior-changes = "off"
```

## Auto-fix Support

Rules marked with "Yes" in the Auto-fix column provide automatic fixes. Run mlt with `--fix` to apply them:

```bash
mlt --fix src/**/*.m
```

Fixes are applied atomically per file. Overlapping fixes are detected and the conflicting fix is skipped with a warning.

## Adding New Rules

mlt uses auto-registration via the `inventory` crate. To add a new rule:

1. Create a new module in `crates/mlt_rules/src/` (e.g., `agrow.rs`)
2. Implement the `Rule` trait with a `from_config` factory
3. Add `inventory::submit!(crate::RuleRegistration::new("AGROW", Agrow::from_config));` at the bottom
4. Add documentation in `docs/rules/agrow.md`

No manual edits to `lib.rs` are needed beyond adding the `pub mod` declaration.

See the [repository](https://github.com/watermarkhu/mlt) for the full development guide.
