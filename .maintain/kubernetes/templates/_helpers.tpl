{{/*
Expand the name of the chart.
*/}}
{{- define "debio-node.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "debio-node.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "debio-node.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "debio-node.labels" -}}
helm.sh/chart: {{ include "debio-node.chart" . }}
{{ include "debio-node.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "debio-node.selectorLabels" -}}
app.kubernetes.io/name: {{ include "debio-node.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "debio-node.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "debio-node.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Create the name of the service http-rpc
*/}}
{{- define "debio-node.serviceHttpRPCName" -}}
{{- printf "%s-%s" (include "debio-node.fullname" .) "http-rpc" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create the name of the service websocket-rpc
*/}}
{{- define "debio-node.serviceWebsocketRPCName" -}}
{{- printf "%s-%s" (include "debio-node.fullname" .) "websocket-rpc" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create the name of the ingress http-rpc
*/}}
{{- define "debio-node.ingressHttpRPCName" -}}
{{- printf "%s-%s" (include "debio-node.fullname" .) "http-rpc" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create the name of the ingress websocket-rpc
*/}}
{{- define "debio-node.ingressWebsocketRPCName" -}}
{{- printf "%s-%s" (include "debio-node.fullname" .) "websocket-rpc" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create the name of session injection job.
*/}}
{{- define "debio-node.sessionInjectionJobName" -}}
{{- printf "%s-%s" (include "debio-node.fullname" .) "session-injection" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create the name of node key secret.
*/}}
{{- define "debio-node.nodeKeySecretName" -}}
{{- printf "%s-%s" (include "debio-node.fullname" .) "node-key" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create the name of session key secret.
*/}}
{{- define "debio-node.sessionKeySecretName" -}}
{{- printf "%s-%s" (include "debio-node.fullname" .) "session-key" | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}
