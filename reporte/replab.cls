% Este template fue originalmente diseñado para emplearse en los
% diferentes cursos de laboratorio en la Facultad de Ciencias de la
% UNAM. El template incluye un archivo de bibliografía .bib que
% contiene la información de ciertos libros de referencia. Aunque esta
% plantilla está equipada con todos los paquetes y funciones que al
% autor le parecieron pertinentes, se le extiende la invitación al
% usuario de personalizar y mejorar cualquier aspecto que le parezca
% necesario. Cualquier comentario o sugerencia de mejora será
% bienvenida en el correo del autor: axelpadilla@ciencias.unam.mx

\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{replab}[2022/12/20 v1.0]

% --- Configuración básica ---

\LoadClass[10pt, letterpaper, twoside, twocolumn]{article}

% Configuración de idioma
\RequirePackage[english, spanish, mexico, es-noitemize, es-noindentfirst, ]{babel}

% Matemáticas avanzadas
\RequirePackage[tbtags]{mathtools}% Funciones matemáticas avanzadas (carga amsmath internamente)
\RequirePackage{amssymb, amsthm, siunitx}% Funciones matemáticas adicionales y unidades del SI
\RequirePackage[nolimits]{cmupint}% Integrales rectas

% Configuración de fuente
\RequirePackage{lmodern}
\RequirePackage[T1]{fontenc}
\renewcommand{\ttdefault}{lmtt}% Fuente mono
\renewcommand{\rmdefault}{lmr}% Fuente serif
\renewcommand{\sfdefault}{lmss}% Fuente sans-serif

% Diseño del documento
\RequirePackage[margin=1in, headheight=2\baselineskip, headsep=\baselineskip, twocolumn]{geometry}
\RequirePackage[pdfusetitle, colorlinks]{hyperref}
\RequirePackage[shortlabels, inline]{enumitem}
\RequirePackage[titles]{tocloft}
\RequirePackage{fancyhdr, tabularray, titlesec, apptools, abstract, xcolor, xpatch, calc, etoolbox, abstract}

% Gráficos
\RequirePackage{graphicx, pdfpages, pdflscape, pgfplots, tcolorbox, graphbox}
\RequirePackage[outline]{contour}
\RequirePackage[hang, labelfont=bf, labelsep=period, margin=0.5in]{caption}
\RequirePackage{subcaption}

% Bibliografía
\RequirePackage[backend=biber]{biblatex}
\RequirePackage[style=mexican]{csquotes}

% --- Configuraciones adicionales de paquetes ---

% Versión de PGFplots
\pgfplotsset{compat=newest}

% Librerías de pgfplots
\usepgfplotslibrary{colormaps}

% Librerías de tikz
\usetikzlibrary{babel, calc, scopes, intersections, angles, quotes, arrows, arrows.meta, backgrounds, shapes.geometric, patterns, shadows, perspective, external}

% Definición de estilos de Tikz, etc.
\tikzset{%
	% Configuraciones de tikz particulares del documento, como estilos de nodos y dibujos
	font = {\selectfont},
	frame/.style={thin, dashed, lightgray, line cap=round, rounded corners=5pt}
}

% Grosor de contorno adicional (para emplearse en gráficos)
\contourlength{1pt}

% Directorios de gráficos
\graphicspath{{img/}}

% Todos los enlaces del documento en negro
\hypersetup{allcolors=black}

% Librerías de tabularray
\UseTblrLibrary{booktabs}
\UseTblrLibrary{amsmath}

% --- Configuraciones adicionales de diseño ---

% Colores
\definecolor{principaluno}{HTML}{003D64}% Azul oscuro
\definecolor{principaldos}{HTML}{EAEDEF}% Gris claro
\definecolor{secuno}{HTML}{0897D5}% Azul claro
\definecolor{secdos}{HTML}{E8442A}% Naranja
\definecolor{sectres}{HTML}{7F9297}% Gris oscuro
\definecolor{seccuatro}{HTML}{F6F7F7}% Casi blanco

% Diseño de títulos de sección
\titleformat{\section}
{\centering\normalfont\Large\bfseries}{\thesection.}{1ex}{\color{principaluno}}

\titleformat{\subsection}
{\normalfont\large\bfseries}{\thesubsection.}{1ex}{\color{principaluno}}

\titleformat{\subsubsection}
{\normalsize\bfseries}{\thetitle.}{\widthof{\enspace}}{\normalsize\color{principaluno}}

\AtAppendix{%
    \titleformat{\section}[block]
    {\centering\normalfont\Large\bfseries}{\appendixname\ \thesection.}{1ex}{\color{principaluno}}
}

% Listas personalizadas
\setlist[enumerate, 1]{%
	label={\textbf{\color{principaluno}\arabic*.}},
	ref=\textbf{\arabic*}
}
\setlist[enumerate, 2]{%
	label={\textbf{\color{principaluno}(\alph*)}},
	ref=\textbf{(\alph*)}
}
\setlist[enumerate, 3]{%
	label={\color{principaluno}\roman*.},
	ref=\roman*
}

\setlist[itemize, 1]{%
	label={\color{principaluno}\textbullet},
}
\setlist[itemize, 2]{%
	label={\color{principaluno}--},
}
\setlist[itemize, 3]{%
	label={\color{principaluno}*},
}

% Redefinición de "y otros" en la bibliografía
\DefineBibliographyStrings{spanish}{andothers={et~al\adddot}}

% Tokens para llenar el documento
\newtoks\subtitle
\newtoks\subject
\newtoks\email

% Personalización del título de tablas largas con tabularray
\NewTblrTheme{replab}{
	\SetTblrStyle{caption-tag}{font=\bfseries}
	\DefTblrTemplate{caption-sep}{default}{\bfseries.\enskip}
}

% --- Página de título de documento ---
\makeatletter
	\renewcommand\maketitle{%

    	\renewcommand\thefootnote{\@fnsymbol\c@footnote}%
    	\def\@makefnmark{\rlap{\@textsuperscript{\normalfont\@thefnmark}}}%
    	\long\def\@makefntext##1{\parindent 1em\noindent
    			\hb@xt@1.8em{%
    			  \hss\@textsuperscript{\normalfont\@thefnmark}}##1}%
    
        \begin{center}%
    		\begin{tblr}{colspec={Q[c, b, wd=.15\textwidth] Q[c, b, wd=.6\textwidth] Q[c, b, wd=.15\textwidth]}}
    			\includegraphics[width=.15\textwidth]{unam}
    			& {\bfseries\large\textcolor{principaluno}{\@title} \\ \LARGE\the\subtitle}
    			& \includegraphics[width=.15\textwidth]{fcunam}
    		\end{tblr}
    		
    		\medskip
    
    % --- Opción para un solo autor (predeterminada) ---
    
    		\@author\\
    		{\footnotesize\the\email}
    
    % --- Opción para dos autores ---
    
            % \begin{tabular}{c@{\hspace{3em}}c}
            %     Nombre del autor 1\thanks{Información adicional del primer autor.} & Nombre del autor 2\thanks{Información adicional del segundo autor.} \\ 
            %     \href{mailto:emailautor1@email.com}{\footnotesize\color{principaluno}\texttt{emailautor1@email.com}} & \href{mailto:emailautor2@email.com}{\footnotesize\color{principaluno}\texttt{emailautor2@email.com}}
            % \end{tabular}
    
    % --- Opción para tres autores ---

            % \begin{tabular}{c@{\hspace{3em}}c@{\hspace{3em}}c}
            %     Nombre del autor 1\thanks{Información adicional del primer autor.} & Nombre del autor 2\thanks{Información adicional del segundo autor.} & Nombre del autor 3\thanks{Información adicional del tercer autor.} \\ 
            %     \href{mailto:emailautor1@email.com}{\footnotesize\color{principaluno}\texttt{emailautor1@email.com}} & \href{mailto:emailautor2@email.com}{\footnotesize\color{principaluno}\texttt{emailautor2@email.com}} & \href{mailto:emailautor3@email.com}{\footnotesize\color{principaluno}\texttt{emailautor3@email.com}}
            % \end{tabular}
    
    % --- Opción para cuatro autores ---

            % \begin{tabular}{c@{\hspace{3em}}c}
            %     Nombre del autor 1\thanks{Información adicional del primer autor.} & Nombre del autor 2\thanks{Información adicional del segundo autor.} \\ 
            %     \href{mailto:emailautor1@email.com}{\footnotesize\color{principaluno}\texttt{emailautor1@email.com}} & \href{mailto:emailautor2@email.com}{\footnotesize\color{principaluno}\texttt{emailautor2@email.com}} \\[10pt]
            %     Nombre del autor 3\thanks{Información adicional del tercer autor.} & Nombre del autor 4\thanks{Información adicional del cuarto autor.} \\
            %     \href{mailto:emailautor3@email.com}{\footnotesize\color{principaluno}\texttt{emailautor3@email.com}} & \href{mailto:emailautor4@email.com}{\footnotesize\color{principaluno}\texttt{emailautor4@email.com}}
            % \end{tabular}
    
    		\medskip
    		
    		Facultad de Ciencias\\
    		Universidad Nacional Autónoma de México
    		
    		\medskip
    		
    		\textit{Fecha de entrega: \@date\\[1ex]}
    
        \end{center}
    
        \thispagestyle{plain}
    
    	\medskip
	}
\makeatother

% Rediseño del estilo plain
\fancypagestyle{plain}{%
	\renewcommand{\headrulewidth}{0pt}
	
	\fancyhf{}
	\fancyfoot[c]{%
		\begin{tblr}{c c c}
			\textcolor{principaluno}{$\boldsymbol{-}$}
			& \thepage &
			\textcolor{principaluno}{$\boldsymbol{-}$}
		\end{tblr}
	}
}

% Diseño de los encabezados y pies de página
\fancypagestyle{fancy}{%
    \xpretocmd\headrule{\color{principaluno}}{}{\PatchFailed}% Cambio de color de la regla en el encabezado
    
    \fancyhf{}
    \fancyhead[le]{%
    	\begin{tblr}{
			colspec = {Q[c,m] Q[l,m]},
			colsep = 2pt
		}
    		\includegraphics[
				align=c,
				width=3ex]{fcunam}% Escudo de la instittución
    		& {\small{Facultad de Ciencias UNAM}}
    	\end{tblr}
    }
    \fancyhead[re]{%
    	\small\textit{\the\subject}% Nombre de la asignatura
    }
    
    \fancyhead[co]{%
        \begin{tblr}{c c c}
            \phantom{\includegraphics[width=3ex]{fcunam}} & \small\textit{\the\subtitle} & \phantom{\includegraphics[width=3ex]{fcunam}}
        \end{tblr}
    }
    
    \fancyfoot[c]{%
    	\begin{tblr}{c c c}
    		\textcolor{principaluno}{$\boldsymbol{-}$}
    		& \thepage &
    		\textcolor{principaluno}{$\boldsymbol{-}$}
    	\end{tblr}
    }
}

% --- Comandos de mate adicionales ---

\newcommand{\pqty}[1]{%
    \left( #1 \right)
}

\newcommand{\bqty}[1]{%
    \left[ #1 \right]
}

\newcommand{\Bqty}[1]{%
    \left\lbrace #1 \right\rbrace
}

\newcommand{\abs}[1]{%
    \left\lvert #1 \right\rvert
}

\newcommand{\norm}[1]{%
    \left\lvert\left\lvert #1 \right\rvert\right\rvert
}

\newcommand{\rtwovec}[2]{% Vectores columna en R^2 
	\begin{bmatrix*}[r]
		#1 \\ #2 
	\end{bmatrix*}
}

\newcommand{\rthreevec}[3]{% Vectores columna en R^3
	\begin{bmatrix*}[r]
		#1 \\ #2 \\ #3
	\end{bmatrix*}
}

\newcommand{\bvec}[1]{% Vectores
	\mathbf{#1}
}

\newcommand{\uvec}[1]{% Vectores unitarios (con gorrito)
	\textbf{\^{#1}}
}

\newenvironment{amatrix}[2]{% Matices aumentadas
	\left[\begin{array}{@{}*{#1}{r}|*{#2}{r}@{}}
	}%
	{
	\end{array}\right]
}